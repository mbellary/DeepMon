use crate::{
    PrototypeError, Result,
    derive::{self, CounterSnapshot, CountingBoundary, LogicalCount},
    endpoint,
    fixture::{self, ValidatedBundle},
    metric_input::{self, CounterRecord, RecordKind, ScenarioInput, Snapshot},
    observation::{self, DerivedResult, NativeObservation, QualityState, SourceContext},
};
use serde::Serialize;
use serde_json::{Value, json};
use std::{collections::BTreeMap, str::FromStr};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MetricScenario {
    NativeParse,
    CounterRate,
    AggregateEvidence,
    CountBoundary,
    MemoryOccupancy,
    VisibilityPolicy,
    FreshnessEvidence,
}
impl MetricScenario {
    pub fn group(self) -> &'static str {
        match self {
            Self::NativeParse => "PX01",
            Self::CounterRate => "PX02",
            Self::AggregateEvidence => "PX03",
            Self::CountBoundary => "PX04",
            Self::MemoryOccupancy => "PX05",
            Self::VisibilityPolicy => "PX11",
            Self::FreshnessEvidence => "PX12",
        }
    }
    pub fn for_group(group: &str) -> Option<Self> {
        [
            Self::NativeParse,
            Self::CounterRate,
            Self::AggregateEvidence,
            Self::CountBoundary,
            Self::MemoryOccupancy,
            Self::VisibilityPolicy,
            Self::FreshnessEvidence,
        ]
        .into_iter()
        .find(|s| s.group() == group)
    }
}
impl FromStr for MetricScenario {
    type Err = PrototypeError;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "native-parse" => Ok(Self::NativeParse),
            "counter-rate" => Ok(Self::CounterRate),
            "aggregate-evidence" => Ok(Self::AggregateEvidence),
            "count-boundary" => Ok(Self::CountBoundary),
            "memory-occupancy" => Ok(Self::MemoryOccupancy),
            "visibility-policy" => Ok(Self::VisibilityPolicy),
            "freshness-evidence" => Ok(Self::FreshnessEvidence),
            _ => Err(PrototypeError::new(
                "not_prototyped",
                "scenario is not implemented",
            )),
        }
    }
}
#[derive(Debug, Serialize)]
pub struct MetricScenarioReport {
    pub schema_version: u32,
    pub synthetic: bool,
    pub scenario: MetricScenario,
    pub variant_id: String,
    pub bundle_id: String,
    pub manifest_sha256: String,
    pub input_sha256: String,
    pub acquisition_kind: &'static str,
    pub functional_execution_status: &'static str,
    pub source_input_ids: Vec<String>,
    pub native_observations: Vec<NativeObservation>,
    pub reported_input_records: Vec<Value>,
    pub results: Vec<DerivedResult>,
    pub not_evaluated_record_ids: Vec<String>,
    pub limits_of_claim: String,
    pub px_runtime_acceptance_status: &'static str,
    pub real_verification: &'static str,
    pub runtime_acceptance_readiness: bool,
    pub production_support_claimed: bool,
}
fn invalid() -> PrototypeError {
    PrototypeError::new("input_invalid", "missing or incompatible typed record data")
}
fn required<T: serde::de::DeserializeOwned>(r: &metric_input::Record, key: &str) -> Result<T> {
    serde_json::from_value(r.get(key).ok_or_else(invalid)?.clone()).map_err(|_| invalid())
}
fn has_kind(r: &metric_input::Record, kind: RecordKind) -> bool {
    std::mem::discriminant(&r.kind) == std::mem::discriminant(&kind)
}
fn projected(input: &ScenarioInput) -> Vec<Value> {
    input
        .records
        .iter()
        .map(|r| {
            let mut v = json!({"id":r.id,"kind":r.kind});
            let obj = v.as_object_mut().unwrap();
            for name in [
                "family",
                "incarnation",
                "value",
                "time_us",
                "clock_domain",
                "unit",
                "count",
                "producer",
                "scope",
                "used",
                "free",
                "total",
                "device",
                "source_status",
                "optional_family",
                "statistics_active",
                "family_present",
                "buckets",
                "sum",
                "success_count",
                "missing_joint_records",
            ] {
                if let Some(value) = r.get(name) {
                    obj.insert(name.into(), value.clone());
                }
            }
            observation::sanitize(&mut v);
            v
        })
        .collect()
}
pub fn run_metric_scenario(
    bundle: &ValidatedBundle,
    scenario: MetricScenario,
    variant_id: &str,
) -> Result<MetricScenarioReport> {
    let group = bundle
        .manifest
        .fixtures
        .iter()
        .find(|g| g.variants.iter().any(|v| v.id == variant_id))
        .ok_or_else(|| PrototypeError::new("variant_unknown", "unknown variant"))?;
    if group.experiment_id != scenario.group() {
        return Err(PrototypeError::new(
            "scenario_incompatible",
            "scenario does not apply to selected group",
        ));
    }
    let input = fixture::load_scenario_input(bundle, variant_id)?;
    let mut report = MetricScenarioReport {
        schema_version: 1,
        synthetic: true,
        scenario,
        variant_id: variant_id.into(),
        bundle_id: bundle.manifest.bundle_id.clone(),
        manifest_sha256: bundle.manifest_sha256.clone(),
        input_sha256: input.sha256.clone(),
        acquisition_kind: "synthetic_record_input",
        functional_execution_status: "executed",
        source_input_ids: group.pinned_source_ids.clone(),
        native_observations: Vec::new(),
        reported_input_records: projected(&input),
        results: Vec::new(),
        not_evaluated_record_ids: Vec::new(),
        limits_of_claim: group.limits_of_claim.clone(),
        px_runtime_acceptance_status: "blocked_unagreed_budget",
        real_verification: "not_run",
        runtime_acceptance_readiness: false,
        production_support_claimed: false,
    };
    match scenario {
        MetricScenario::NativeParse => {
            if let Some(r) = input
                .records
                .iter()
                .find(|r| has_kind(r, RecordKind::EndpointSnapshot))
            {
                let snapshot: Snapshot = r.decode()?;
                let source = SourceContext {
                    family: snapshot.family.clone(),
                    producer: snapshot.producer.clone(),
                    scope: snapshot.scope.clone(),
                    unit: snapshot.unit.clone(),
                    unit_provenance: "explicit_snapshot_metadata".into(),
                    fixture_id: variant_id.into(),
                    record_id: r.id.clone(),
                    source_input_ids: snapshot.source_input_ids,
                    publisher: None,
                    subject: None,
                    time_us: Some(snapshot.time_us),
                    clock_domain: Some(snapshot.clock_domain.clone()),
                    clock_uncertainty_us: input
                        .clock(&snapshot.clock_domain)
                        .map(|c| c.uncertainty_us),
                };
                let endpoint_input = fixture::select_endpoint(bundle, variant_id)?;
                let server = endpoint::spawn_fixture_server(
                    endpoint_input.clone(),
                    endpoint::ServerConfig::default(),
                )?;
                let bytes = match endpoint::fetch_fixture(&server) {
                    Ok(b) => b,
                    Err(e) => {
                        let _ = server.shutdown_and_join();
                        return Err(e);
                    }
                };
                let outcome = server.wait()?;
                if outcome.rejected_requests != 0 || bytes != endpoint_input.bytes() {
                    return Err(PrototypeError::new(
                        "transport_mismatch",
                        "endpoint transfer failed",
                    ));
                }
                let text = std::str::from_utf8(&bytes).map_err(|_| invalid())?;
                let observations = endpoint::parse_metric_fixture(text, &source)?;
                for native in observations {
                    if native.mapped {
                        let typ =
                            serde_json::to_value(&native.metric_type).map_err(|_| invalid())?;
                        if typ != snapshot.metric_type {
                            return Err(PrototypeError::new(
                                "TYPE_INVALID",
                                "snapshot and declared type differ",
                            ));
                        }
                        let values: Vec<_> = native.samples.iter().map(|s| s.value).collect();
                        report.results.push(derive::state(
                            &native.family,
                            json!({"type":typ,"scope":native.source.scope,"sample_values":values}),
                            &native.source.unit,
                            QualityState::ReportedOnly,
                            "native_source_reported",
                            vec![r.id.clone()],
                        ));
                    }
                    report
                        .native_observations
                        .push(observation::safe_native(native));
                }
                report.acquisition_kind = "owned_loopback_endpoint";
            } else {
                let absent = input
                    .records
                    .iter()
                    .any(|r| r.get("family_present") == Some(&json!(false)));
                if !absent {
                    return Err(invalid());
                }
                report.results.push(DerivedResult::unknown(
                    "trt_iteration_latency",
                    "optional_family_absent",
                ));
                report.acquisition_kind = "explicit_absence_no_endpoint";
            }
        }
        MetricScenario::CounterRate => {
            let counters: Vec<_> = input
                .records
                .iter()
                .filter(|r| has_kind(r, RecordKind::Counter))
                .collect();
            if counters.is_empty() {
                return Err(invalid());
            }
            if counters
                .iter()
                .any(|r| r.get("family") == Some(&json!("tgi_request_count")))
            {
                report.results.push(DerivedResult::unknown(
                    "accepted_arrival_rate",
                    "population_unresolved",
                ));
            } else if counters.len() != 2 {
                report.results.push(DerivedResult::unknown(
                    "source_success_rate",
                    "missing_snapshot",
                ));
            } else {
                let make = |r: &metric_input::Record| -> Result<CounterSnapshot> {
                    let c: CounterRecord = r.decode()?;
                    if c.family != "nv_inference_request_success" {
                        return Err(invalid());
                    }
                    let clock = input.clock(&c.clock_domain).ok_or_else(invalid)?;
                    Ok(CounterSnapshot {
                        id: r.id.clone(),
                        family: c.family,
                        producer: "triton".into(),
                        scope: "fixture_source".into(),
                        population: "source_success_requests".into(),
                        labels: BTreeMap::new(),
                        incarnation: c.incarnation,
                        unit: c.unit,
                        value: c.value,
                        time_us: c.time_us,
                        clock_domain: c.clock_domain,
                        uncertainty_us: clock.uncertainty_us,
                    })
                };
                report.results.push(derive::counter_rate(
                    &make(counters[0])?,
                    &make(counters[1])?,
                ));
            }
        }
        MetricScenario::AggregateEvidence => {
            if !input.records.iter().any(|r| {
                has_kind(r, RecordKind::Aggregate)
                    && r.get("missing_joint_records") == Some(&json!(true))
            }) {
                return Err(invalid());
            }
            report.results.push(DerivedResult::unknown(
                "exact_goodput_and_variance",
                "insufficient_input",
            ));
        }
        MetricScenario::CountBoundary => {
            let counts: Vec<_> = input
                .records
                .iter()
                .filter(|r| has_kind(r, RecordKind::Aggregate))
                .collect();
            if counts.len() == 2 {
                let make = |r: &metric_input::Record| -> Result<LogicalCount> {
                    Ok(LogicalCount {
                        id: r.id.clone(),
                        producer: required(r, "producer")?,
                        population: "explicit_fixture_logical_requests".into(),
                        count: required(r, "count")?,
                        logical_ids: r
                            .get("logical_ids")
                            .map(|v| serde_json::from_value(v.clone()).map_err(|_| invalid()))
                            .transpose()?,
                    })
                };
                let front = make(counts[0])?;
                let back = make(counts[1])?;
                report.results.push(derive::logical_count(
                    &front,
                    &back,
                    &CountingBoundary {
                        selected_producer: "frontend".into(),
                        population: front.population.clone(),
                        explicit_overlap: true,
                    },
                ));
                if input
                    .records
                    .iter()
                    .any(|r| r.get("subject_host") == Some(&Value::Null))
                {
                    report.results.push(DerivedResult::unknown(
                        "worker_host",
                        "location_evidence_missing",
                    ));
                }
            }
            report.not_evaluated_record_ids = input
                .records
                .iter()
                .filter(|r| {
                    has_kind(r, RecordKind::Identity) || r.get("publisher_candidates").is_some()
                })
                .map(|r| r.id.clone())
                .collect();
        }
        MetricScenario::MemoryOccupancy => {
            for r in &input.records {
                if has_kind(r, RecordKind::Memory) {
                    let m: metric_input::MemoryRecord = r.decode()?;
                    let mut value = derive::memory_occupancy(&m);
                    value.record_ids = vec![r.id.clone()];
                    report.results.push(value);
                } else if has_kind(r, RecordKind::Aggregate) {
                    let count: u64 = required(r, "count")?;
                    let scope: String = required(r, "scope")?;
                    report.results.push(DerivedResult::known(
                        &scope,
                        json!(count),
                        "requests",
                        vec![r.id.clone()],
                    ));
                } else if has_kind(r, RecordKind::Gauge) && r.get("total") == Some(&Value::Null) {
                    report
                        .results
                        .push(DerivedResult::unknown("gpu_occupancy", "missing_total"));
                } else {
                    report.not_evaluated_record_ids.push(r.id.clone());
                }
            }
        }
        MetricScenario::VisibilityPolicy => {
            for r in &input.records {
                if !has_kind(r, RecordKind::Policy) {
                    return Err(invalid());
                }
                if r.get("source_status").is_some() {
                    let source: String = required(r, "source_status")?;
                    let optional: String = required(r, "optional_family")?;
                    if source != "permission_denied" || optional != "not_enabled" {
                        return Err(invalid());
                    }
                    report.results.push(derive::state(
                        "source_status",
                        json!(source),
                        "source_state",
                        QualityState::Denied,
                        "source_permission_denied",
                        vec![r.id.clone()],
                    ));
                    report.results.push(derive::state(
                        "optional_family",
                        json!(optional),
                        "capability_state",
                        QualityState::NotEnabled,
                        "optional_family_disabled",
                        vec![r.id.clone()],
                    ));
                    let labels: BTreeMap<String, String> = required(r, "retained_labels")?;
                    let safe: BTreeMap<_, _> = labels
                        .into_iter()
                        .filter(|(k, v)| k == "fixture" && !v.contains(observation::MARKER))
                        .collect();
                    report.results.push(DerivedResult::known(
                        "retained_labels",
                        json!(safe),
                        "label_map",
                        vec![r.id.clone()],
                    ));
                    report
                        .results
                        .push(DerivedResult::unknown("identity", "visibility_denied"));
                } else if r.get("attempted_actions").is_some() {
                    let actions: Vec<String> = required(r, "attempted_actions")?;
                    if actions.is_empty()
                        || actions.iter().any(|s| {
                            !["enable_metric", "escalate_privilege", "restart_workload"]
                                .contains(&s.as_str())
                        })
                    {
                        return Err(invalid());
                    }
                    report.results.push(derive::state(
                        "actions",
                        json!("rejected"),
                        "policy_decision",
                        QualityState::Denied,
                        "forbidden_action",
                        vec![r.id.clone()],
                    ));
                    if r.get("agreement") != Some(&Value::Null)
                        || r.get("required_limit") != Some(&Value::Null)
                    {
                        return Err(invalid());
                    }
                    report.results.push(derive::state(
                        "runtime_readiness",
                        json!(false),
                        "boolean",
                        QualityState::Blocked,
                        "required_agreement_missing",
                        vec![r.id.clone()],
                    ));
                } else {
                    return Err(invalid());
                }
            }
        }
        MetricScenario::FreshnessEvidence => {
            let gauges: Vec<_> = input
                .records
                .iter()
                .filter(|r| has_kind(r, RecordKind::Gauge))
                .collect();
            if gauges.len() == 2 {
                report.results.push(DerivedResult::unknown(
                    "intervening_source_updates",
                    "latest_gauge_not_event_history",
                ));
            } else if input.records.iter().any(|r| {
                r.get("source_update_time_us") == Some(&Value::Null)
                    || r.get("query_visible_time_us") == Some(&Value::Null)
            }) {
                report.results.push(DerivedResult::unknown(
                    "source_and_query_freshness",
                    "missing_stage_timestamps",
                ));
            } else {
                return Err(invalid());
            }
            report.not_evaluated_record_ids = input
                .records
                .iter()
                .filter(|r| has_kind(r, RecordKind::Queue))
                .map(|r| r.id.clone())
                .collect();
        }
    }
    report.results.sort_by(|a, b| a.target.cmp(&b.target));
    report.not_evaluated_record_ids.sort();
    Ok(report)
}
#[derive(Debug, Serialize)]
pub struct AssertionCheck {
    pub assertion_id: String,
    pub target: String,
    pub kind: String,
    pub assigned_milestone: u32,
    pub functional_execution_status: &'static str,
    pub functional_check_status: &'static str,
    pub reason: String,
}
#[derive(Debug, Serialize)]
pub struct ScenarioVerification {
    pub variant_id: String,
    pub scenario: MetricScenario,
    pub input_sha256: String,
    pub oracle_sha256: String,
    pub result_sha256: Option<String>,
    pub actual: Option<MetricScenarioReport>,
    pub actual_error: Option<String>,
    pub checks: Vec<AssertionCheck>,
    pub passed: bool,
    pub px_runtime_acceptance_status: &'static str,
    pub runtime_acceptance_readiness: bool,
}
fn assigned(group: &str, kind: &str, target: &str) -> u32 {
    match group {
        "PX06" | "PX07" => 5,
        "PX08" | "PX09" => 7,
        "PX10" => 6,
        "PX04" if target == "publisher" => 5,
        "PX05" if target == "composition" => 5,
        "PX12" if kind == "queue_counts" => 6,
        _ => 4,
    }
}
fn numeric_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => {
            if let (Some(a), Some(b)) = (a.as_u64(), b.as_u64()) {
                return a == b;
            }
            if let (Some(a), Some(b)) = (a.as_i64(), b.as_i64()) {
                return a == b;
            }
            if a.as_u64().is_some_and(|n| n > 9_007_199_254_740_991)
                || b.as_u64().is_some_and(|n| n > 9_007_199_254_740_991)
            {
                return false;
            }
            a.as_f64() == b.as_f64()
        }
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| numeric_equal(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(k, v)| b.get(k).is_some_and(|b| numeric_equal(v, b)))
        }
        _ => a == b,
    }
}
/// Independent comparison; it never changes actual results or supplies runtime inputs.
pub fn compare_result(actual: &DerivedResult, expected: &metric_input::Assertion) -> bool {
    actual.target == expected.target
        && serde_json::to_value(actual.state).ok().as_ref() == Some(&json!(expected.state))
        && actual.unit == expected.unit
        && numeric_equal(&actual.value, &expected.expected_value)
}
fn proof_verified(proof: Option<&Value>, expected: &Value) -> bool {
    let Some(p) = proof else {
        return false;
    };
    if p["variance"]["population"] != true
        || p["variance"]["unit"] != "seconds_squared"
        || p["variance"]["buckets_seconds"] != json!([1, "+Inf"])
        || p["goodput"]["unit"] != "requests/second"
    {
        return false;
    }
    let numbers =
        |v: &Value| -> Option<Vec<f64>> { v.as_array()?.iter().map(Value::as_f64).collect() };
    let check = || -> Option<Value> {
        let a = numbers(&p["variance"]["a"])?;
        let b = numbers(&p["variance"]["b"])?;
        if a.len() != 2
            || b.len() != 2
            || a.iter()
                .chain(&b)
                .any(|n| !n.is_finite() || *n < 0.0 || *n > 1.0)
            || a.iter().sum::<f64>() != b.iter().sum::<f64>()
            || a.iter().sum::<f64>() != 1.0
        {
            return None;
        }
        let variance = |v: &Vec<f64>| {
            let mean = v.iter().sum::<f64>() / v.len() as f64;
            v.iter().map(|n| (n - mean).powi(2)).sum::<f64>() / v.len() as f64
        };
        let latencies = numbers(&p["goodput"]["latencies_seconds"])?;
        let slo = p["goodput"]["slo_seconds"].as_f64()?;
        let window = p["goodput"]["window_seconds"].as_f64()?;
        if !window.is_finite()
            || window <= 0.0
            || !slo.is_finite()
            || slo < 0.0
            || latencies.iter().any(|n| !n.is_finite() || *n < 0.0)
        {
            return None;
        }
        let a_flags: Vec<bool> = p["goodput"]["success_a"]
            .as_array()?
            .iter()
            .map(Value::as_bool)
            .collect::<Option<_>>()?;
        let b_flags: Vec<bool> = p["goodput"]["success_b"]
            .as_array()?
            .iter()
            .map(Value::as_bool)
            .collect::<Option<_>>()?;
        if a_flags.len() != latencies.len()
            || b_flags.len() != latencies.len()
            || a_flags.iter().filter(|n| **n).count() != b_flags.iter().filter(|n| **n).count()
        {
            return None;
        }
        let goodput = |flags: Vec<bool>| {
            latencies
                .iter()
                .zip(flags)
                .filter(|(lat, s)| **lat <= slo && *s)
                .count() as f64
                / window
        };
        Some(
            json!({"variance_a":variance(&a),"variance_b":variance(&b),"goodput_a":goodput(a_flags),"goodput_b":goodput(b_flags)}),
        )
    };
    check().is_some_and(|v| {
        numeric_equal(&v, expected)
            && v["variance_a"] != v["variance_b"]
            && v["goodput_a"] != v["goodput_b"]
    })
}
pub fn verify_metric_scenario(
    bundle: &ValidatedBundle,
    scenario: MetricScenario,
    variant_id: &str,
) -> Result<ScenarioVerification> {
    // Actual result is produced before any semantic expected answer is loaded here.
    let actual = run_metric_scenario(bundle, scenario, variant_id);
    let expected = fixture::load_expected(bundle, variant_id)?;
    let input = fixture::load_scenario_input(bundle, variant_id)?;
    let result_hash = actual
        .as_ref()
        .ok()
        .map(|r| {
            serde_json::to_value(r)
                .and_then(|mut v| {
                    observation::sanitize(&mut v);
                    serde_json::to_vec(&v)
                })
                .map(|b| aether_validation::sha256(&b))
        })
        .transpose()
        .map_err(|_| invalid())?;
    let mut checks = Vec::new();
    for (index, e) in expected.assertions.iter().enumerate() {
        let stage = assigned(scenario.group(), &e.kind, &e.target);
        let (status, execution, reason) = if stage != 4 {
            (
                "deferred",
                "not_run",
                format!("assigned to milestone {stage}"),
            )
        } else if e.kind == "proof" {
            if actual.is_ok() && proof_verified(expected.proof.as_ref(), &e.expected_value) {
                (
                    "proof_verified",
                    "proof_only",
                    "oracle counterexample proof; not a runtime observation".into(),
                )
            } else {
                (
                    "mismatched",
                    "proof_only",
                    "counterexample verification failed".into(),
                )
            }
        } else if e.kind == "rejection" {
            if actual
                .as_ref()
                .is_err_and(|error| Some(error.code) == e.expected_value.as_str())
            {
                (
                    "expected_rejection",
                    "executed",
                    "runtime parser returned the precise expected error".into(),
                )
            } else {
                (
                    "mismatched",
                    "executed",
                    "wrong or absent runtime rejection".into(),
                )
            }
        } else if actual
            .as_ref()
            .is_ok_and(|r| r.results.iter().any(|r| compare_result(r, e)))
        {
            (
                "matched",
                "executed",
                "actual target/value/unit/state matches independent expected answer".into(),
            )
        } else {
            (
                "mismatched",
                "executed",
                "required actual result missing or differs; payload omitted".into(),
            )
        };
        checks.push(AssertionCheck {
            assertion_id: format!("{variant_id}:{index}:{}:{}", e.kind, e.target),
            target: e.target.clone(),
            kind: e.kind.clone(),
            assigned_milestone: stage,
            functional_execution_status: execution,
            functional_check_status: status,
            reason,
        });
    }
    let passed = checks
        .iter()
        .all(|c| c.functional_check_status != "mismatched")
        && (actual.is_ok()
            || checks
                .iter()
                .any(|c| c.functional_check_status == "expected_rejection"));
    let (actual, actual_error) = match actual {
        Ok(r) => (Some(r), None),
        Err(e) => (None, Some(e.code.to_string())),
    };
    Ok(ScenarioVerification {
        variant_id: variant_id.into(),
        scenario,
        input_sha256: input.sha256,
        oracle_sha256: expected.sha256,
        result_sha256: result_hash,
        actual,
        actual_error,
        checks,
        passed,
        px_runtime_acceptance_status: "blocked_unagreed_budget",
        runtime_acceptance_readiness: false,
    })
}
pub fn verify_milestone(bundle: &ValidatedBundle) -> Result<Value> {
    let mut groups = Vec::new();
    let mut variants = 0;
    let mut statuses: BTreeMap<String, u64> = BTreeMap::new();
    let mut passed = true;
    for group in &bundle.manifest.fixtures {
        let mut entries = Vec::new();
        for v in &group.variants {
            if let Some(scenario) = MetricScenario::for_group(&group.experiment_id) {
                let report = verify_metric_scenario(bundle, scenario, &v.id)?;
                passed &= report.passed;
                variants += 1;
                for c in &report.checks {
                    *statuses
                        .entry(c.functional_check_status.into())
                        .or_default() += 1;
                }
                entries.push(serde_json::to_value(report).map_err(|_| invalid())?);
            } else {
                let e = fixture::load_expected(bundle, &v.id)?;
                let assertions:Vec<_>=e.assertions.iter().enumerate().map(|(i,a)|json!({"assertion_id":format!("{}:{i}:{}:{}",v.id,a.kind,a.target),"target":a.target,"kind":a.kind,"assigned_milestone":assigned(&group.experiment_id,&a.kind,&a.target),"functional_execution_status":"not_run","functional_check_status":"deferred","reason":"later milestone"})).collect();
                *statuses.entry("not_run_later_stage".into()).or_default() +=
                    assertions.len() as u64;
                entries.push(json!({"variant_id":v.id,"oracle_sha256":e.sha256,"functional_execution_status":"not_run","checks":assertions}));
            }
        }
        groups.push(json!({"experiment_id":group.experiment_id,"requirement_ids":group.requirement_ids,"association_ids":group.association_ids,"variants":entries}));
    }
    let mut summary = json!({"schema_version":1,"synthetic":true,"milestone":4,"manifest_sha256":bundle.manifest_sha256,"functional_groups_considered":7,"functional_variants_considered":variants,"assertion_outcomes":statuses,"functional_checks_passed":passed,"groups":groups,"px_runtime_acceptance_status":"blocked_unagreed_budget","real_verification":"not_run","runtime_acceptance_readiness":false,"production_support_claimed":false});
    observation::sanitize(&mut summary);
    Ok(summary)
}
