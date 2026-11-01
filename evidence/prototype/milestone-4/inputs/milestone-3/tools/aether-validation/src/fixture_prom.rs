//! Deliberately finite sample grammar, not a production exposition parser.
use std::collections::BTreeMap;
#[derive(Debug)]
pub(crate) struct Family {
    pub typ: String,
    pub samples: Vec<Sample>,
}
#[derive(Debug)]
pub(crate) struct Sample {
    pub name: String,
    pub labels: BTreeMap<String, String>,
    pub value: f64,
}
fn name_ok(s: &str, colon: bool) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || (colon && c == ':'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || (colon && c == ':'))
}
pub(crate) fn parse_prom(text: &str) -> Result<BTreeMap<String, Family>, String> {
    let mut families = BTreeMap::<String, Family>::new();
    let mut helps = BTreeMap::new();
    let mut samples = Vec::new();
    let mut unique = std::collections::BTreeSet::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        if let Some(rest) = line.strip_prefix("# HELP ") {
            let (name, help) = rest.split_once(' ').ok_or("HELP_INVALID")?;
            if !name_ok(name, true) || help.is_empty() || helps.insert(name, true).is_some() {
                return Err("HELP_INVALID".into());
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("# TYPE ") {
            let fields: Vec<_> = rest.split_whitespace().collect();
            if fields.len() != 2
                || !name_ok(fields[0], true)
                || !matches!(fields[1], "counter" | "gauge" | "histogram")
            {
                return Err("TYPE_INVALID".into());
            }
            if families
                .insert(
                    fields[0].into(),
                    Family {
                        typ: fields[1].into(),
                        samples: Vec::new(),
                    },
                )
                .is_some()
            {
                return Err("TYPE_INVALID".into());
            }
            continue;
        }
        if line.starts_with('#') {
            return Err("COMMENT_UNSUPPORTED".into());
        }
        let split = line.rfind(char::is_whitespace).ok_or("SAMPLE_INVALID")?;
        let value = line[split..]
            .trim()
            .parse::<f64>()
            .map_err(|_| "SAMPLE_INVALID")?;
        if !value.is_finite() || value < 0.0 {
            return Err("SAMPLE_INVALID".into());
        }
        let lhs = &line[..split];
        let mut labels = BTreeMap::new();
        let name = if let Some((name, body)) = lhs.split_once('{') {
            let body = body.strip_suffix('}').ok_or("LABEL_INVALID")?;
            for part in body.split(',') {
                let (key, raw) = part.split_once('=').ok_or("LABEL_INVALID")?;
                let val = raw
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    .ok_or("LABEL_INVALID")?;
                if !name_ok(key, false)
                    || val.contains(['"', '\\', '{', '}'])
                    || labels.insert(key.to_string(), val.to_string()).is_some()
                {
                    return Err("LABEL_INVALID".into());
                }
            }
            name
        } else {
            lhs
        };
        if !name_ok(name, true) {
            return Err("SAMPLE_INVALID".into());
        }
        let key = (name.to_owned(), labels.clone());
        if !unique.insert(key) {
            return Err("DUPLICATE_SAMPLE".into());
        }
        samples.push(Sample {
            name: name.into(),
            labels,
            value,
        });
    }
    if families.is_empty() {
        return Err("TYPE_MISSING".into());
    }
    for sample in samples {
        let key = if families.contains_key(&sample.name) {
            sample.name.clone()
        } else {
            let (prefix, _) = sample.name.rsplit_once('_').ok_or("TYPE_MISSING")?;
            prefix.to_owned()
        };
        let family = families.get_mut(&key).ok_or("TYPE_MISSING")?;
        if family.typ == "histogram"
            && ![
                format!("{key}_bucket"),
                format!("{key}_count"),
                format!("{key}_sum"),
            ]
            .contains(&sample.name)
        {
            return Err("HISTOGRAM_INCONSISTENT".into());
        }
        if family.typ != "histogram" && sample.name != key {
            return Err("SAMPLE_INVALID".into());
        }
        family.samples.push(sample);
    }
    for (name, f) in &families {
        if !helps.contains_key(name.as_str()) || f.samples.is_empty() {
            return Err("HELP_OR_SAMPLE_MISSING".into());
        }
        if f.typ == "histogram" {
            let mut sets = BTreeMap::<BTreeMap<String, String>, Vec<&Sample>>::new();
            for sample in &f.samples {
                let mut labels = sample.labels.clone();
                labels.remove("le");
                sets.entry(labels).or_default().push(sample);
            }
            for group in sets.values() {
                let mut buckets = Vec::new();
                let mut count = None;
                let mut sum = None;
                for s in group {
                    if s.name.ends_with("_bucket") {
                        let le = s.labels.get("le").ok_or("HISTOGRAM_INCONSISTENT")?;
                        let bound = if le == "+Inf" {
                            f64::INFINITY
                        } else {
                            let f = le.parse::<f64>().map_err(|_| "HISTOGRAM_INCONSISTENT")?;
                            if !f.is_finite() {
                                return Err("HISTOGRAM_INCONSISTENT".into());
                            }
                            f
                        };
                        if s.value.fract() != 0.0 {
                            return Err("HISTOGRAM_INCONSISTENT".into());
                        }
                        buckets.push((bound, s.value));
                    } else if s.name.ends_with("_count") {
                        if s.labels.contains_key("le")
                            || count.replace(s.value).is_some()
                            || s.value.fract() != 0.0
                        {
                            return Err("HISTOGRAM_INCONSISTENT".into());
                        }
                    } else if s.name.ends_with("_sum")
                        && (s.labels.contains_key("le") || sum.replace(s.value).is_some())
                    {
                        return Err("HISTOGRAM_INCONSISTENT".into());
                    }
                }
                buckets.sort_by(|a, b| a.0.total_cmp(&b.0));
                if buckets.len() < 2
                    || buckets
                        .windows(2)
                        .any(|p| p[0].0 >= p[1].0 || p[0].1 > p[1].1)
                    || buckets.last().map(|p| p.0) != Some(f64::INFINITY)
                    || buckets.last().map(|p| p.1) != count
                    || count.is_none()
                    || sum.is_none()
                {
                    return Err("HISTOGRAM_INCONSISTENT".into());
                }
            }
        }
    }
    Ok(families)
}
