use crate::{
    metric_input::MemoryRecord,
    observation::{DerivedResult, QualityState},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterSnapshot {
    pub id: String,
    pub family: String,
    pub producer: String,
    pub scope: String,
    pub population: String,
    pub labels: BTreeMap<String, String>,
    pub incarnation: String,
    pub unit: String,
    pub value: u64,
    pub time_us: u64,
    pub clock_domain: String,
    pub uncertainty_us: u64,
}
pub fn counter_rate(previous: &CounterSnapshot, current: &CounterSnapshot) -> DerivedResult {
    let unknown = |r| DerivedResult::unknown("source_success_rate", r);
    if previous.family != current.family
        || previous.producer != current.producer
        || previous.scope != current.scope
        || previous.population != current.population
        || previous.labels != current.labels
        || previous.unit != current.unit
    {
        return unknown("population_mismatch");
    }
    if previous.incarnation != current.incarnation || current.value < previous.value {
        return unknown("reset_or_incarnation_changed");
    }
    if previous.clock_domain != current.clock_domain
        || previous.uncertainty_us != 0
        || current.uncertainty_us != 0
    {
        return unknown("clock_uncertain");
    }
    let Some(duration) = current
        .time_us
        .checked_sub(previous.time_us)
        .filter(|d| *d > 0)
    else {
        return unknown("nonpositive_interval");
    };
    if previous.value > 9_007_199_254_740_991
        || current.value > 9_007_199_254_740_991
        || duration > 9_007_199_254_740_991
    {
        return unknown("counter_precision");
    }
    if previous.unit != "requests" || previous.population != "source_success_requests" {
        return unknown("population_unresolved");
    }
    let rate = (current.value - previous.value) as f64 / (duration as f64 / 1_000_000.0);
    let mut result = DerivedResult::known(
        "source_success_rate",
        json!(rate),
        "requests/second",
        vec![previous.id.clone(), current.id.clone()],
    );
    result.population = Some(previous.population.clone());
    result.scope = Some(previous.scope.clone());
    result.clock_domain = Some(previous.clock_domain.clone());
    result.interval_us = Some([previous.time_us, current.time_us]);
    result
}
pub fn memory_occupancy(record: &MemoryRecord) -> DerivedResult {
    let Some(total) = record.total else {
        return DerivedResult::unknown("gpu_occupancy", "missing_total");
    };
    if record.free.is_some_and(|free| {
        !free.is_finite() || free < 0.0 || free > total || free + record.used != total
    }) || !record.used.is_finite()
        || !total.is_finite()
        || record.used < 0.0
        || total <= 0.0
        || record.used > total
        || record.unit != "MiB"
    {
        return DerivedResult::unknown("gpu_occupancy", "invalid_denominator_or_unit");
    }
    DerivedResult::known(
        &record.device,
        json!(record.used / total),
        "ratio",
        Vec::new(),
    )
}
#[derive(Debug, Clone)]
pub struct LogicalCount {
    pub id: String,
    pub producer: String,
    pub population: String,
    pub count: u64,
    pub logical_ids: Option<Vec<String>>,
}
#[derive(Debug, Clone)]
pub struct CountingBoundary {
    pub selected_producer: String,
    pub population: String,
    pub explicit_overlap: bool,
}
pub fn logical_count(
    front: &LogicalCount,
    back: &LogicalCount,
    boundary: &CountingBoundary,
) -> DerivedResult {
    let unknown = || DerivedResult::unknown("selected_boundary", "insufficient_linkage");
    if !boundary.explicit_overlap
        || boundary.selected_producer != front.producer
        || boundary.population != front.population
        || front.population != back.population
    {
        return unknown();
    }
    let (Some(a), Some(b)) = (&front.logical_ids, &back.logical_ids) else {
        return unknown();
    };
    let a_set: BTreeSet<_> = a.iter().collect();
    let b_set: BTreeSet<_> = b.iter().collect();
    if a_set.len() != a.len()
        || b_set.len() != b.len()
        || a_set != b_set
        || a.len() as u64 != front.count
        || b.len() as u64 != back.count
    {
        return unknown();
    }
    let mut r = DerivedResult::known(
        "selected_boundary",
        json!(front.count),
        "requests",
        vec![front.id.clone(), back.id.clone()],
    );
    r.population = Some(boundary.population.clone());
    r.scope = Some(boundary.selected_producer.clone());
    r
}
pub fn state(
    target: &str,
    value: serde_json::Value,
    unit: &str,
    state: QualityState,
    reason: &str,
    ids: Vec<String>,
) -> DerivedResult {
    let mut r = DerivedResult::known(target, value, unit, ids);
    r.state = state;
    r.reason_code = reason.into();
    r
}
