//! Finite synthetic observation interchange. No physical capture guarantees.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceContext {
    pub family: String,
    pub producer: String,
    pub scope: String,
    pub unit: String,
    pub unit_provenance: String,
    pub fixture_id: String,
    pub record_id: String,
    pub source_input_ids: Vec<String>,
    pub publisher: Option<String>,
    pub subject: Option<String>,
    pub time_us: Option<u64>,
    pub clock_domain: Option<String>,
    pub clock_uncertainty_us: Option<u64>,
}
#[derive(Debug, Clone, Serialize)]
pub struct NativeSample {
    pub name: String,
    pub labels: BTreeMap<String, String>,
    pub value: f64,
}
#[derive(Debug, Clone, Serialize)]
pub struct NativeObservation {
    pub family: String,
    pub metric_type: MetricType,
    pub help: Option<String>,
    pub samples: Vec<NativeSample>,
    pub source: SourceContext,
    pub mapped: bool,
    pub redacted_label_keys: Vec<String>,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QualityState {
    Known,
    ReportedOnly,
    Unknown,
    Unavailable,
    Ambiguous,
    Partial,
    Denied,
    NotEnabled,
    Blocked,
    NotPrototyped,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedResult {
    pub target: String,
    pub value: Value,
    pub unit: String,
    pub state: QualityState,
    pub reason_code: String,
    pub record_ids: Vec<String>,
    pub population: Option<String>,
    pub scope: Option<String>,
    pub clock_domain: Option<String>,
    pub interval_us: Option<[u64; 2]>,
}
impl DerivedResult {
    pub fn known(target: &str, value: Value, unit: &str, ids: Vec<String>) -> Self {
        Self {
            target: target.into(),
            value,
            unit: unit.into(),
            state: QualityState::Known,
            reason_code: "supplied_evidence".into(),
            record_ids: ids,
            population: None,
            scope: None,
            clock_domain: None,
            interval_us: None,
        }
    }
    pub fn unknown(target: &str, reason: &str) -> Self {
        let mut r = Self::known(target, Value::Null, "not_available", Vec::new());
        r.state = QualityState::Unknown;
        r.reason_code = reason.into();
        r
    }
}
pub const MARKER: &str = "SYNTHETIC_ONLY_SECRET_MARKER";
/// Final safe JSON boundary for all metric output, including unexpected strings.
pub fn sanitize(value: &mut Value) {
    match value {
        Value::String(s) => {
            *s = s.replace(MARKER, "[REDACTED]");
        }
        Value::Array(a) => a.iter_mut().for_each(sanitize),
        Value::Object(m) => {
            let old = std::mem::take(m);
            for (key, mut v) in old {
                sanitize(&mut v);
                m.insert(key.replace(MARKER, "[REDACTED]"), v);
            }
        }
        _ => (),
    }
}
pub fn safe_native(mut native: NativeObservation) -> NativeObservation {
    for s in &mut native.samples {
        for (key, value) in &mut s.labels {
            if ![
                "model_name",
                "model",
                "version",
                "memory_type",
                "le",
                "fixture",
            ]
            .contains(&key.as_str())
                || value.contains(MARKER)
            {
                native.redacted_label_keys.push(key.clone());
                *value = "[REDACTED]".into();
            }
        }
    }
    native.redacted_label_keys.sort();
    native.redacted_label_keys.dedup();
    native
}
