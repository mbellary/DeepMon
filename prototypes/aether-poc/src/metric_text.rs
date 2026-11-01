//! Independent runtime parser for the finite synthetic exposition grammar.
use crate::{
    PrototypeError, Result,
    observation::{MetricType, NativeObservation, NativeSample, SourceContext},
};
use std::collections::{BTreeMap, BTreeSet};
type RawValues = BTreeMap<(String, BTreeMap<String, String>), String>;
fn exact_count(raw: &str) -> bool {
    let raw = raw.strip_suffix(".0").unwrap_or(raw);
    !raw.is_empty()
        && raw.bytes().all(|b| b.is_ascii_digit())
        && raw.parse::<u64>().is_ok_and(|n| n <= 9_007_199_254_740_991)
}
fn err(code: &'static str) -> PrototypeError {
    PrototypeError::new(code, "finite metric grammar violation; payload omitted")
}
fn name_ok(s: &str, colon: bool) -> bool {
    let mut c = s.chars();
    c.next()
        .is_some_and(|v| v.is_ascii_alphabetic() || v == '_' || (colon && v == ':'))
        && c.all(|v| v.is_ascii_alphanumeric() || v == '_' || (colon && v == ':'))
}
pub fn parse(text: &str, source: &SourceContext) -> Result<Vec<NativeObservation>> {
    if text.len() > 1024 * 1024 {
        return Err(err("size_limit"));
    }
    let mut types = BTreeMap::new();
    let mut helps = BTreeMap::new();
    let mut samples = Vec::new();
    let mut unique = BTreeSet::new();
    let mut raw_values = RawValues::new();
    for line in text.lines().filter(|s| !s.trim().is_empty()) {
        if let Some(rest) = line.strip_prefix("# HELP ") {
            let (name, value) = rest.split_once(' ').ok_or_else(|| err("HELP_INVALID"))?;
            if !name_ok(name, true)
                || value.is_empty()
                || helps.insert(name.to_string(), value.to_string()).is_some()
            {
                return Err(err("HELP_INVALID"));
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("# TYPE ") {
            let parts: Vec<_> = rest.split_whitespace().collect();
            if parts.len() != 2 || !name_ok(parts[0], true) {
                return Err(err("TYPE_INVALID"));
            }
            let typ = match parts[1] {
                "counter" => MetricType::Counter,
                "gauge" => MetricType::Gauge,
                "histogram" => MetricType::Histogram,
                _ => return Err(err("TYPE_INVALID")),
            };
            if types.insert(parts[0].to_string(), typ).is_some() {
                return Err(err("TYPE_INVALID"));
            }
            continue;
        }
        if line.starts_with('#') {
            return Err(err("SYNTAX_UNSUPPORTED"));
        }
        let (lhs, raw) = line
            .rsplit_once(char::is_whitespace)
            .ok_or_else(|| err("SAMPLE_INVALID"))?;
        if lhs.chars().any(char::is_whitespace) {
            return Err(err("SAMPLE_INVALID"));
        }
        let value: f64 = raw.parse().map_err(|_| err("SAMPLE_INVALID"))?;
        if !value.is_finite() || value < 0.0 {
            return Err(err("SAMPLE_INVALID"));
        }
        let mut labels = BTreeMap::new();
        let name = if let Some((name, body)) = lhs.split_once('{') {
            let body = body.strip_suffix('}').ok_or_else(|| err("LABEL_INVALID"))?;
            if !body.is_empty() {
                for pair in body.split(',') {
                    let (key, raw) = pair.split_once('=').ok_or_else(|| err("LABEL_INVALID"))?;
                    let val = raw
                        .strip_prefix('"')
                        .and_then(|v| v.strip_suffix('"'))
                        .ok_or_else(|| err("LABEL_INVALID"))?;
                    if !name_ok(key, false)
                        || val.len() > 256
                        || val.contains(['"', '\\', '{', '}'])
                        || labels.insert(key.to_string(), val.to_string()).is_some()
                    {
                        return Err(err("LABEL_INVALID"));
                    }
                }
            }
            name
        } else {
            lhs
        };
        if !name_ok(name, true) || !unique.insert((name.to_string(), labels.clone())) {
            return Err(err("SAMPLE_INVALID"));
        }
        raw_values.insert((name.to_string(), labels.clone()), raw.to_string());
        samples.push(NativeSample {
            name: name.into(),
            labels,
            value,
        });
        if samples.len() > 64 {
            return Err(err("sample_limit"));
        }
    }
    if types.len() > 64 || helps.keys().any(|n| !types.contains_key(n)) {
        return Err(err("TYPE_INVALID"));
    }
    let mut grouped: BTreeMap<String, Vec<NativeSample>> =
        types.keys().map(|s| (s.clone(), Vec::new())).collect();
    for sample in samples {
        let name = if types.contains_key(&sample.name) {
            sample.name.clone()
        } else {
            ["_bucket", "_count", "_sum"]
                .iter()
                .filter_map(|suffix| sample.name.strip_suffix(suffix))
                .find(|base| types.get(*base) == Some(&MetricType::Histogram))
                .ok_or_else(|| err("TYPE_INVALID"))?
                .to_string()
        };
        if types[&name] == MetricType::Counter
            && !exact_count(&raw_values[&(sample.name.clone(), sample.labels.clone())])
        {
            return Err(err("COUNTER_PRECISION"));
        }
        grouped.get_mut(&name).unwrap().push(sample);
    }
    let mut result = Vec::new();
    for (family, typ) in types {
        let samples = grouped.remove(&family).unwrap();
        if typ == MetricType::Histogram {
            validate_histogram(&family, &samples, &raw_values)?;
        }
        let mapped = family == source.family;
        let mut context = source.clone();
        if !mapped {
            context.family = family.clone();
            context.unit = "unknown".into();
            context.unit_provenance = "unmapped_native_family".into();
            context.subject = None;
        }
        result.push(NativeObservation {
            family: family.clone(),
            metric_type: typ,
            help: helps.remove(&family),
            samples,
            source: context,
            mapped,
            redacted_label_keys: Vec::new(),
        });
    }
    if result.is_empty() {
        return Err(err("TYPE_INVALID"));
    }
    Ok(result)
}
fn validate_histogram(
    family: &str,
    samples: &[NativeSample],
    raw_values: &RawValues,
) -> Result<()> {
    #[derive(Default)]
    struct Group {
        buckets: Vec<(f64, f64)>,
        count: Option<f64>,
        sum: Option<f64>,
    }
    let mut groups: BTreeMap<BTreeMap<String, String>, Group> = BTreeMap::new();
    for s in samples {
        if (s.name == format!("{family}_bucket") || s.name == format!("{family}_count"))
            && !exact_count(&raw_values[&(s.name.clone(), s.labels.clone())])
        {
            return Err(err("HISTOGRAM_INCONSISTENT"));
        }
        let mut labels = s.labels.clone();
        let le = labels.remove("le");
        let g = groups.entry(labels).or_default();
        if s.name == format!("{family}_bucket") {
            let bound = le.ok_or_else(|| err("HISTOGRAM_INCONSISTENT"))?;
            let bound = if bound == "+Inf" {
                f64::INFINITY
            } else {
                let n: f64 = bound.parse().map_err(|_| err("HISTOGRAM_INCONSISTENT"))?;
                if !n.is_finite() {
                    return Err(err("HISTOGRAM_INCONSISTENT"));
                }
                n
            };
            if s.value.fract() != 0.0 || s.value > 9_007_199_254_740_991.0 {
                return Err(err("HISTOGRAM_INCONSISTENT"));
            }
            g.buckets.push((bound, s.value));
        } else if s.name == format!("{family}_count") && le.is_none() {
            if s.value.fract() != 0.0 || g.count.replace(s.value).is_some() {
                return Err(err("HISTOGRAM_INCONSISTENT"));
            }
        } else if s.name == format!("{family}_sum") && le.is_none() {
            if g.sum.replace(s.value).is_some() {
                return Err(err("HISTOGRAM_INCONSISTENT"));
            }
        } else {
            return Err(err("HISTOGRAM_INCONSISTENT"));
        }
    }
    if groups.is_empty() {
        return Err(err("HISTOGRAM_INCONSISTENT"));
    }
    for g in groups.values() {
        if g.sum.is_none()
            || g.count.is_none()
            || g.buckets
                .last()
                .is_none_or(|(b, c)| *b != f64::INFINITY || Some(*c) != g.count)
            || g.buckets
                .windows(2)
                .any(|w| w[0].0 >= w[1].0 || w[0].1 > w[1].1)
        {
            return Err(err("HISTOGRAM_INCONSISTENT"));
        }
    }
    Ok(())
}
