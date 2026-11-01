use crate::{PrototypeError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Clock {
    pub domain: String,
    pub unit: String,
    pub uncertainty_us: u64,
    pub offset_bounds_us: Option<[i64; 2]>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioInput {
    pub schema_version: u32,
    pub synthetic: bool,
    pub variant_id: String,
    pub clocks: Vec<Clock>,
    pub records: Vec<Record>,
    #[serde(skip)]
    pub sha256: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordKind {
    EndpointSnapshot,
    Counter,
    Gauge,
    Aggregate,
    Identity,
    Link,
    Memory,
    Policy,
    Queue,
}
#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: String,
    pub kind: RecordKind,
    #[serde(flatten)]
    pub fields: BTreeMap<String, Value>,
}
impl Record {
    pub fn decode<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        let value = serde_json::to_value(&self.fields)
            .map_err(|_| PrototypeError::new("input_invalid", "record serialization"))?;
        serde_json::from_value(value).map_err(|_| {
            PrototypeError::new(
                "input_invalid",
                format!("invalid fields for record {}", self.id),
            )
        })
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }
}
impl ScenarioInput {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || !self.synthetic
            || self.records.len() > 64
            || self.clocks.len() > 8
        {
            return Err(PrototypeError::new(
                "input_invalid",
                "invalid envelope/limits",
            ));
        }
        let mut ids = BTreeSet::new();
        let mut clocks = BTreeSet::new();
        for c in &self.clocks {
            if c.unit != "microseconds"
                || !clocks.insert(&c.domain)
                || c.offset_bounds_us.is_some_and(|b| b[0] > b[1])
            {
                return Err(PrototypeError::new("input_invalid", "invalid clock"));
            }
        }
        for r in &self.records {
            if r.id.is_empty() || !ids.insert(&r.id) {
                return Err(PrototypeError::new(
                    "input_invalid",
                    "duplicate/empty record id",
                ));
            }
            let allowed: &[&str] = match r.kind {
                RecordKind::EndpointSnapshot => &[
                    "clock_domain",
                    "family",
                    "invented_fields",
                    "invented_labels",
                    "metric_type",
                    "native_labels",
                    "producer",
                    "scope",
                    "source_input_ids",
                    "time_us",
                    "unit",
                ],
                RecordKind::Counter => &[
                    "clock_domain",
                    "family",
                    "incarnation",
                    "time_us",
                    "unit",
                    "value",
                ],
                RecordKind::Gauge => &[
                    "scope",
                    "clock_domain",
                    "family",
                    "time_us",
                    "unit",
                    "value",
                    "total",
                    "family_present",
                    "statistics_active",
                ],
                RecordKind::Aggregate => &[
                    "producer",
                    "count",
                    "logical_ids",
                    "scope",
                    "buckets",
                    "success_count",
                    "sum",
                    "unit",
                    "missing_joint_records",
                ],
                RecordKind::Identity => &[
                    "entity",
                    "incarnation",
                    "valid_from_us",
                    "valid_to_us",
                    "evidence",
                ],
                RecordKind::Link => &[
                    "process_record",
                    "container",
                    "cgroup",
                    "valid_from_us",
                    "valid_to_us",
                    "capabilities",
                    "configured_preference",
                    "driver",
                    "loaded_cuda_nccl",
                    "model",
                    "operation_path",
                    "is_payload_path",
                    "protocol",
                    "socket",
                    "publisher_candidates",
                    "model_alias",
                    "subject",
                    "subject_host",
                    "emitter",
                    "evidence",
                    "members",
                    "complete",
                    "replica",
                    "models",
                    "other_replica_same_model_alias",
                    "capture_time_us",
                    "source_update_time_us",
                    "persisted_time_us",
                    "query_visible_time_us",
                ],
                RecordKind::Memory => &["total", "used", "free", "device", "unit"],
                RecordKind::Policy => &[
                    "container",
                    "source",
                    "status",
                    "source_status",
                    "optional_family",
                    "identity",
                    "synthetic_sensitive_marker",
                    "retained_labels",
                    "attempted_actions",
                    "agreement",
                    "required_limit",
                    "runtime_readiness",
                ],
                RecordKind::Queue => &[
                    "policy",
                    "capacity_records",
                    "admitted_ids",
                    "rejected_ids",
                    "offered_ids",
                ],
            };
            if r.fields.keys().any(|k| !allowed.contains(&k.as_str())) {
                return Err(PrototypeError::new(
                    "input_invalid",
                    "unsupported record field",
                ));
            }
            if let Some(domain) = r.get("clock_domain")
                && !domain
                    .as_str()
                    .is_some_and(|d| clocks.contains(&d.to_string()))
            {
                return Err(PrototypeError::new("input_invalid", "unknown clock domain"));
            }
        }
        Ok(())
    }
    pub fn clock(&self, domain: &str) -> Option<&Clock> {
        self.clocks.iter().find(|c| c.domain == domain)
    }
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub clock_domain: String,
    pub family: String,
    pub invented_fields: Vec<String>,
    pub invented_labels: Vec<String>,
    pub metric_type: String,
    pub native_labels: Vec<String>,
    pub producer: String,
    pub scope: String,
    pub source_input_ids: Vec<String>,
    pub time_us: u64,
    pub unit: String,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CounterRecord {
    pub clock_domain: String,
    pub family: String,
    pub incarnation: String,
    pub time_us: u64,
    pub unit: String,
    pub value: u64,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryRecord {
    pub total: Option<f64>,
    pub used: f64,
    pub free: Option<f64>,
    pub device: String,
    pub unit: String,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedAssertions {
    pub schema_version: u32,
    pub synthetic: bool,
    pub variant_id: String,
    pub experiment_execution_status: String,
    pub assertions: Vec<Assertion>,
    pub proof: Option<Value>,
    #[serde(skip)]
    pub sha256: String,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assertion {
    pub kind: String,
    pub target: String,
    pub expected_value: Value,
    pub unit: String,
    pub state: String,
    pub record_ids: Vec<String>,
    pub reason: String,
}
