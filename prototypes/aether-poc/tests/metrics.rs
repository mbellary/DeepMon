use aether_poc::{
    derive::{self, CounterSnapshot, CountingBoundary, LogicalCount},
    endpoint,
    fixture::{self, ValidatedBundle},
    metric_input::{self, MemoryRecord},
    observation::{self, DerivedResult, QualityState, SourceContext},
    scenario::{self, MetricScenario},
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn bundle() -> ValidatedBundle {
    fixture::load_bundle(&root(), Path::new("fixtures/prototype/manifest.json")).unwrap()
}
fn source() -> SourceContext {
    SourceContext {
        family: "m".into(),
        producer: "fixture".into(),
        scope: "fixture_server".into(),
        unit: "requests".into(),
        unit_provenance: "test_explicit".into(),
        fixture_id: "test".into(),
        record_id: "snapshot".into(),
        source_input_ids: vec![],
        publisher: None,
        subject: None,
        time_us: Some(0),
        clock_domain: Some("fixture".into()),
        clock_uncertainty_us: Some(0),
    }
}
fn run(s: MetricScenario, v: &str) -> scenario::MetricScenarioReport {
    scenario::run_metric_scenario(&bundle(), s, v).unwrap()
}
fn result(report: &scenario::MetricScenarioReport, target: &str) -> DerivedResult {
    report
        .results
        .iter()
        .find(|r| r.target == target)
        .unwrap()
        .clone()
}
fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_aether-poc"))
        .current_dir(root())
        .args(args)
        .output()
        .unwrap()
}
fn tree() -> tempfile::TempDir {
    let t = tempfile::tempdir().unwrap();
    let source = root();
    let m: Value =
        serde_json::from_slice(&fs::read(source.join("evidence/manifest-v1.json")).unwrap())
            .unwrap();
    let mut files: Vec<_> = m["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["path"].as_str().unwrap().to_string())
        .collect();
    files.extend([
        "evidence/manifest-v1.json".into(),
        "evidence/manifest-v1.sha256".into(),
    ]);
    for f in files {
        let target = t.path().join(&f);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(source.join(f), target).unwrap();
    }
    aether_validation::generate_records(t.path()).unwrap();
    aether_validation::author_fixtures(t.path()).unwrap();
    t
}
#[test]
fn all_seven_native_producers_preserve_independent_oracles() {
    let b = bundle();
    for id in [
        "PX01-vllm-v1",
        "PX01-sglang-v1",
        "PX01-dynamo-v1",
        "PX01-tgi-v1",
        "PX01-triton-v1",
        "PX01-trt-native-v1",
        "PX01-trt-backend-v1",
    ] {
        let v = scenario::verify_metric_scenario(&b, MetricScenario::NativeParse, id).unwrap();
        assert!(v.passed, "{id}: {:?}", v.checks);
        assert_eq!(
            v.actual.unwrap().acquisition_kind,
            "owned_loopback_endpoint"
        );
    }
}
#[test]
fn histogram_total_suffix_never_overrides_declared_type() {
    let report = run(MetricScenario::NativeParse, "PX01-vllm-v1");
    let native = &report.native_observations[0];
    assert_eq!(native.family, "vllm:iteration_tokens_total");
    assert_eq!(native.metric_type, observation::MetricType::Histogram);
    assert!(native.samples[0].name.ends_with("_bucket"));
    assert_eq!(native.samples[0].labels["model_name"], "fixture-model");
    assert!(native.source.clock_uncertainty_us.is_some());
}
#[test]
fn server_pool_scope_and_backend_labels_stay_native() {
    let triton = run(MetricScenario::NativeParse, "PX01-triton-v1");
    assert_eq!(
        triton.native_observations[0].source.scope,
        "server_all_models"
    );
    let backend = run(MetricScenario::NativeParse, "PX01-trt-backend-v1");
    assert_eq!(
        backend.native_observations[0].samples[0].labels["memory_type"],
        "gpu"
    );
    assert!(backend.native_observations[0].source.subject.is_none());
}
#[test]
fn malformed_payloads_fail_runtime_and_verify_only_precise_rejection() {
    let b = bundle();
    for (id, code) in [
        ("PX01-malformed-type-v1", "TYPE_INVALID"),
        ("PX01-inconsistent-histogram-v1", "HISTOGRAM_INCONSISTENT"),
    ] {
        assert_eq!(
            scenario::run_metric_scenario(&b, MetricScenario::NativeParse, id)
                .unwrap_err()
                .code,
            code
        );
        let v = scenario::verify_metric_scenario(&b, MetricScenario::NativeParse, id).unwrap();
        assert!(v.passed);
        assert_eq!(v.checks[0].functional_check_status, "expected_rejection");
        assert!(v.actual.is_none());
    }
}
#[test]
fn optional_absence_is_not_a_zero_or_network_acquisition() {
    let r = run(MetricScenario::NativeParse, "PX01-optional-absent-v1");
    assert_eq!(r.acquisition_kind, "explicit_absence_no_endpoint");
    assert!(r.native_observations.is_empty());
    assert_eq!(r.results[0].state, QualityState::Unknown);
    assert!(r.results[0].value.is_null());
}
#[test]
fn unknown_families_retained_without_invented_unit_or_mapping() {
    let out = endpoint::parse_metric_fixture(
        "# TYPE unknown_family gauge\nunknown_family 7\n",
        &source(),
    )
    .unwrap();
    assert_eq!(out[0].family, "unknown_family");
    assert!(!out[0].mapped);
    assert_eq!(out[0].source.unit, "unknown");
}
#[test]
fn parser_rejects_duplicate_labels_series_types_and_unsupported_syntax() {
    for text in [
        "# TYPE m gauge\nm{x=\"a\",x=\"b\"} 1\n",
        "# TYPE m gauge\nm 1\nm 2\n",
        "# TYPE m gauge\n# TYPE m counter\nm 1\n",
        "m 1\n",
        "# TYPE m summary\nm 1\n",
        "# TYPE m gauge\nm 1 1000\n",
        "# TYPE m gauge\nm{x=\"a,b\"} 1\n",
        "# TYPE m gauge\nm NaN\n",
        "# TYPE m gauge\nm +Inf\n",
        "# TYPE m gauge\nm -1\n",
    ] {
        assert!(
            endpoint::parse_metric_fixture(text, &source()).is_err(),
            "accepted {text}"
        );
    }
}
#[test]
fn parser_bounds_bytes_labels_samples_and_counter_precision() {
    let texts = [
        " ".repeat(1024 * 1024 + 1),
        format!("# TYPE m gauge\nm{{x=\"{}\"}} 1\n", "x".repeat(257)),
        format!(
            "# TYPE m gauge\n{}",
            (0..65)
                .map(|i| format!("m{{fixture=\"{i}\"}} 1\n"))
                .collect::<String>()
        ),
        "# TYPE m counter\nm 9007199254740993\n".into(),
        "# TYPE m counter\nm 9007199254740991.1\n".into(),
    ];
    for text in texts {
        assert!(endpoint::parse_metric_fixture(&text, &source()).is_err());
    }
}
#[test]
fn histograms_validate_each_label_population_independently() {
    let good = "# TYPE m histogram\nm_bucket{model=\"a\",le=\"1\"} 1\nm_bucket{model=\"a\",le=\"+Inf\"} 2\nm_count{model=\"a\"} 2\nm_sum{model=\"a\"} 3\n";
    assert!(endpoint::parse_metric_fixture(good, &source()).is_ok());
    for bad in [
        good.replace("m_count{model=\"a\"} 2", "m_count{model=\"b\"} 2"),
        good.replace("le=\"+Inf\"} 2", "le=\"+Inf\"} 0"),
        good.replace("le=\"+Inf\"", "le=\"0.5\""),
        good.replace("m_sum{model=\"a\"} 3\n", ""),
    ] {
        assert_eq!(
            endpoint::parse_metric_fixture(&bad, &source())
                .unwrap_err()
                .code,
            "HISTOGRAM_INCONSISTENT"
        );
    }
}
fn counter(value: u64, time_us: u64) -> CounterSnapshot {
    CounterSnapshot {
        id: format!("c{time_us}"),
        family: "nv_inference_request_success".into(),
        producer: "triton".into(),
        scope: "fixture_source".into(),
        population: "source_success_requests".into(),
        labels: BTreeMap::new(),
        incarnation: "one".into(),
        unit: "requests".into(),
        value,
        time_us,
        clock_domain: "fixture".into(),
        uncertainty_us: 0,
    }
}
#[test]
fn rate_uses_supplied_snapshot_population_and_time() {
    let r = run(MetricScenario::CounterRate, "PX02-success-rate-v1");
    let v = result(&r, "source_success_rate");
    assert_eq!(v.value, json!(5.0));
    assert_eq!(v.unit, "requests/second");
    assert_eq!(v.interval_us, Some([0, 10_000_000]));
    assert_eq!(r.acquisition_kind, "synthetic_record_input");
    assert_eq!(r.reported_input_records[0]["value"], 100);
    assert_eq!(r.reported_input_records[1]["value"], 150);
}
#[test]
fn reset_and_raw_tgi_semantics_remain_unknown() {
    for (id, target) in [
        ("PX02-reset-v1", "source_success_rate"),
        ("PX02-raw-tgi-boundary-v1", "accepted_arrival_rate"),
    ] {
        let r = run(MetricScenario::CounterRate, id);
        assert_eq!(result(&r, target).state, QualityState::Unknown);
        assert!(result(&r, target).value.is_null());
        assert!(!r.runtime_acceptance_readiness);
    }
}
#[test]
fn rate_rejects_reset_bad_time_population_units_labels_and_clocks() {
    let before = counter(100, 0);
    let valid = counter(150, 10_000_000);
    assert_eq!(derive::counter_rate(&before, &valid).value, json!(5.0));
    let mut cases = Vec::new();
    for i in 0..11 {
        let mut after = valid.clone();
        match i {
            0 => after.value = 99,
            1 => after.incarnation = "two".into(),
            2 => after.time_us = 0,
            3 => after.scope = "other".into(),
            4 => after.unit = "tokens".into(),
            5 => after.population = "all_requests".into(),
            6 => after
                .labels
                .insert("model".into(), "other".into())
                .map(|_| ())
                .unwrap_or(()),
            7 => after.clock_domain = "remote".into(),
            8 => after.uncertainty_us = 1,
            9 => after.value = 9_007_199_254_740_992,
            _ => after.producer = "other".into(),
        };
        cases.push(after);
    }
    for after in cases {
        assert_eq!(
            derive::counter_rate(&before, &after).state,
            QualityState::Unknown
        );
    }
    assert_eq!(
        derive::counter_rate(&counter(100, 100), &counter(101, 99)).reason_code,
        "nonpositive_interval"
    );
}
fn logical(producer: &str) -> LogicalCount {
    LogicalCount {
        id: producer.into(),
        producer: producer.into(),
        population: "requests".into(),
        count: 2,
        logical_ids: Some(vec!["r1".into(), "r2".into()]),
    }
}
#[test]
fn explicit_overlap_selects_one_boundary_and_rejects_missing_linkage() {
    let a = logical("frontend");
    let mut b = logical("backend");
    let boundary = CountingBoundary {
        selected_producer: "frontend".into(),
        population: "requests".into(),
        explicit_overlap: true,
    };
    assert_eq!(derive::logical_count(&a, &b, &boundary).value, 2);
    b.logical_ids = None;
    assert_eq!(
        derive::logical_count(&a, &b, &boundary).state,
        QualityState::Unknown
    );
    b.logical_ids = Some(vec!["r1".into(), "r1".into()]);
    assert_eq!(
        derive::logical_count(&a, &b, &boundary).state,
        QualityState::Unknown
    );
}
#[test]
fn overlap_does_not_infer_remote_worker_host() {
    let r = run(MetricScenario::CountBoundary, "PX04-overlap-and-remote-v1");
    assert_eq!(result(&r, "selected_boundary").value, 10);
    assert_eq!(result(&r, "worker_host").state, QualityState::Unknown);
    assert_eq!(
        r.reported_input_records
            .iter()
            .filter_map(|r| r["count"].as_u64())
            .sum::<u64>(),
        20
    );
}
#[test]
fn memory_ratios_preserve_single_replica_count() {
    let r = run(MetricScenario::MemoryOccupancy, "PX05-replica-set-v1");
    assert_eq!(result(&r, "gpu-A").value, json!(0.5));
    assert_eq!(result(&r, "gpu-B").value, json!(0.5));
    assert_eq!(
        r.results.iter().filter(|r| r.target == "replica-A").count(),
        1
    );
    assert_eq!(result(&r, "replica-A").value, 100);
}
#[test]
fn memory_missing_invalid_denominators_and_units_do_not_create_capacity() {
    let mut record = MemoryRecord {
        total: Some(20.0),
        used: 10.0,
        free: Some(10.0),
        device: "gpu".into(),
        unit: "MiB".into(),
    };
    assert_eq!(derive::memory_occupancy(&record).value, json!(0.5));
    for total in [None, Some(0.0), Some(-1.0), Some(5.0)] {
        record.total = total;
        assert_eq!(
            derive::memory_occupancy(&record).state,
            QualityState::Unknown
        );
    }
    record.total = Some(20.0);
    record.unit = "bytes".into();
    assert_eq!(
        derive::memory_occupancy(&record).state,
        QualityState::Unknown
    );
    let r = run(
        MetricScenario::MemoryOccupancy,
        "PX05-partial-membership-v1",
    );
    assert_eq!(result(&r, "gpu_occupancy").state, QualityState::Unknown);
}
#[test]
fn aggregate_runtime_cannot_see_oracle_worlds() {
    for id in ["PX03-aggregate-proof-v1", "PX03-insufficient-joint-data-v1"] {
        let r = run(MetricScenario::AggregateEvidence, id);
        assert_eq!(
            result(&r, "exact_goodput_and_variance").state,
            QualityState::Unknown
        );
        let text = serde_json::to_string(&r).unwrap();
        assert!(!text.contains("variance_a"));
        assert!(!text.contains("\"goodput_a\":"));
        assert_eq!(r.reported_input_records[0]["sum"], 1);
        assert!(r.reported_input_records[0]["buckets"].is_array());
    }
}
#[test]
fn proof_verification_stays_separate_from_runtime() {
    let v = scenario::verify_metric_scenario(
        &bundle(),
        MetricScenario::AggregateEvidence,
        "PX03-aggregate-proof-v1",
    )
    .unwrap();
    assert!(v.passed);
    assert_eq!(v.checks[0].functional_check_status, "proof_verified");
    assert_eq!(v.checks[0].functional_execution_status, "proof_only");
    assert_eq!(v.actual.unwrap().results[0].state, QualityState::Unknown);
}
#[test]
fn privacy_denial_and_readiness_are_independent_states() {
    let r = run(
        MetricScenario::VisibilityPolicy,
        "PX11-visibility-and-redaction-v1",
    );
    assert_eq!(result(&r, "source_status").state, QualityState::Denied);
    assert_eq!(
        result(&r, "optional_family").state,
        QualityState::NotEnabled
    );
    assert_eq!(result(&r, "identity").state, QualityState::Unknown);
    assert_eq!(
        result(&r, "retained_labels").value,
        json!({"fixture":"safe"})
    );
    assert!(
        !serde_json::to_string(&r)
            .unwrap()
            .contains(observation::MARKER)
    );
    let r = run(
        MetricScenario::VisibilityPolicy,
        "PX11-forbidden-actions-v1",
    );
    assert_eq!(result(&r, "actions").state, QualityState::Denied);
    assert_eq!(result(&r, "runtime_readiness").value, false);
}
#[test]
fn native_projection_removes_sensitive_label_values() {
    let text = format!(
        "# HELP m {}\n# TYPE m gauge\nm{{secret=\"{}\",model=\"fixture\"}} 1\n",
        observation::MARKER,
        observation::MARKER
    );
    let raw = endpoint::parse_metric_fixture(&text, &source())
        .unwrap()
        .remove(0);
    assert_eq!(raw.samples[0].labels["secret"], observation::MARKER);
    let safe = observation::safe_native(raw);
    assert_eq!(safe.samples[0].labels["secret"], "[REDACTED]");
    assert!(safe.redacted_label_keys.contains(&"secret".into()));
    let mut value = serde_json::to_value(safe).unwrap();
    observation::sanitize(&mut value);
    assert!(!value.to_string().contains(observation::MARKER));
}
#[test]
fn latest_gauges_and_missing_stage_times_cannot_prove_freshness() {
    for (id, target) in [
        ("PX12-gauge-and-queue-v1", "intervening_source_updates"),
        (
            "PX12-unknown-persistence-freshness-v1",
            "source_and_query_freshness",
        ),
    ] {
        let r = run(MetricScenario::FreshnessEvidence, id);
        assert_eq!(result(&r, target).state, QualityState::Unknown);
        assert!(r.results.iter().all(|r| r.target != "q"));
    }
}
#[test]
fn all_assertions_accounted_and_runtime_acceptance_stays_blocked() {
    let summary = scenario::verify_milestone(&bundle()).unwrap();
    assert_eq!(summary["functional_groups_considered"], 7);
    assert_eq!(summary["functional_variants_considered"], 23);
    assert_eq!(summary["groups"].as_array().unwrap().len(), 12);
    assert_eq!(
        summary["assertion_outcomes"],
        json!({"matched":24,"expected_rejection":2,"proof_verified":1,"deferred":3,"not_run_later_stage":19})
    );
    assert_eq!(summary["functional_checks_passed"], true);
    assert_eq!(summary["runtime_acceptance_readiness"], false);
    assert!(!summary.to_string().contains(observation::MARKER));
}
#[test]
fn oracle_changes_cannot_feed_runtime_and_are_rejected_by_loader() {
    let t = tree();
    let b = fixture::load_bundle(t.path(), Path::new("fixtures/prototype/manifest.json")).unwrap();
    fs::write(
        t.path()
            .join("fixtures/prototype/expected/PX02-success-rate-v1.json"),
        "{}",
    )
    .unwrap();
    let actual =
        scenario::run_metric_scenario(&b, MetricScenario::CounterRate, "PX02-success-rate-v1")
            .unwrap();
    assert_eq!(result(&actual, "source_success_rate").value, json!(5.0));
    assert_eq!(
        fixture::load_expected(&b, "PX02-success-rate-v1")
            .unwrap_err()
            .code,
        "artifact_changed"
    );
}
#[test]
fn independent_comparison_rejects_value_unit_state_and_large_integer_mismatch() {
    let b = bundle();
    let expected = fixture::load_expected(&b, "PX02-success-rate-v1").unwrap();
    let actual = result(
        &run(MetricScenario::CounterRate, "PX02-success-rate-v1"),
        "source_success_rate",
    );
    assert!(scenario::compare_result(&actual, &expected.assertions[0]));
    for i in 0..3 {
        let mut bad = actual.clone();
        match i {
            0 => bad.value = json!(6),
            1 => bad.unit = "tokens/second".into(),
            _ => bad.state = QualityState::ReportedOnly,
        };
        assert!(!scenario::compare_result(&bad, &expected.assertions[0]));
    }
    let mut a = actual.clone();
    a.value = json!(9_007_199_254_740_992_u64);
    let mut e:metric_input::Assertion=serde_json::from_value(json!({"kind":"rate","target":"source_success_rate","expected_value":9_007_199_254_740_993_u64,"unit":"requests/second","state":"known","record_ids":[],"reason":"test"})).unwrap();
    assert!(!scenario::compare_result(&a, &e));
    e.expected_value = a.value.clone();
    assert!(scenario::compare_result(&a, &e));
}
#[test]
fn record_unknown_fields_ids_and_clocks_are_rejected() {
    for change in ["field", "id", "clock"] {
        let t = tree();
        let b =
            fixture::load_bundle(t.path(), Path::new("fixtures/prototype/manifest.json")).unwrap();
        let mut input = fixture::load_scenario_input(&b, "PX02-success-rate-v1").unwrap();
        match change {
            "field" => {
                input.records[0]
                    .fields
                    .insert("invented_command".into(), json!("ignored"));
            }
            "id" => input.records[1].id = input.records[0].id.clone(),
            _ => {
                input.records[0]
                    .fields
                    .insert("clock_domain".into(), json!("unknown"));
            }
        }
        assert_eq!(input.validate().unwrap_err().code, "input_invalid");
    }
}
#[test]
fn scenario_pairing_and_cli_errors_fail_without_success() {
    assert_eq!(
        scenario::run_metric_scenario(&bundle(), MetricScenario::CounterRate, "PX01-vllm-v1")
            .unwrap_err()
            .code,
        "scenario_incompatible"
    );
    for args in [
        vec![
            "run-scenario",
            "--scenario",
            "token-rate",
            "--variant",
            "PX02-success-rate-v1",
        ],
        vec!["verify-milestone", "--milestone", "5"],
        vec![
            "run-scenario",
            "--scenario",
            "counter-rate",
            "--variant",
            "PX01-vllm-v1",
        ],
    ] {
        let out = cli(&args);
        assert!(!out.status.success());
        assert!(out.stdout.is_empty());
    }
    let out = cli(&[
        "verify-scenario",
        "--scenario",
        "native-parse",
        "--variant",
        "PX01-malformed-type-v1",
    ]);
    assert!(out.status.success());
    assert_eq!(
        serde_json::from_slice::<Value>(&out.stdout).unwrap()["actual_error"],
        "TYPE_INVALID"
    );
}
#[test]
fn cli_outputs_and_errors_never_emit_sensitive_marker() {
    let out = cli(&[
        "run-scenario",
        "--scenario",
        "visibility-policy",
        "--variant",
        "PX11-visibility-and-redaction-v1",
    ]);
    assert!(out.status.success());
    for bytes in [&out.stdout, &out.stderr] {
        assert!(!String::from_utf8_lossy(bytes).contains(observation::MARKER));
    }
    let flag = format!("--{}", observation::MARKER);
    let out = cli(&["list-fixtures", &flag, "bad"]);
    assert!(!out.status.success());
    assert!(!String::from_utf8_lossy(&out.stderr).contains(observation::MARKER));
}
#[test]
fn metric_summary_and_run_commands_are_deterministic() {
    for args in [
        vec![
            "run-scenario",
            "--scenario",
            "counter-rate",
            "--variant",
            "PX02-success-rate-v1",
        ],
        vec!["verify-milestone", "--milestone", "4"],
    ] {
        let a = cli(&args);
        let b = cli(&args);
        assert!(a.status.success());
        assert!(b.status.success());
        assert_eq!(a.stdout, b.stdout);
    }
}

fn edit_expected(b: &mut ValidatedBundle, id: &str, change: impl FnOnce(&mut Value)) {
    let a = b
        .manifest
        .fixtures
        .iter_mut()
        .flat_map(|g| &mut g.variants)
        .find(|v| v.id == id)
        .unwrap()
        .artifacts
        .iter_mut()
        .find(|a| a.role == "expected")
        .unwrap();
    let path = b.root.join("fixtures/prototype").join(&a.path);
    let mut value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    change(&mut value);
    let bytes = serde_json::to_vec_pretty(&value).unwrap();
    a.bytes = bytes.len() as u64;
    a.sha256 = aether_validation::sha256(&bytes);
    fs::write(path, bytes).unwrap();
}
#[test]
fn wrong_expected_rejection_never_matches_a_different_parser_error() {
    let t = tree();
    let mut b =
        fixture::load_bundle(t.path(), Path::new("fixtures/prototype/manifest.json")).unwrap();
    edit_expected(&mut b, "PX01-malformed-type-v1", |v| {
        v["assertions"][0]["expected_value"] = json!("HISTOGRAM_INCONSISTENT")
    });
    let v =
        scenario::verify_metric_scenario(&b, MetricScenario::NativeParse, "PX01-malformed-type-v1")
            .unwrap();
    assert!(!v.passed);
    assert_eq!(v.actual_error.as_deref(), Some("TYPE_INVALID"));
}
#[test]
fn proof_metadata_corruption_is_a_failure_not_a_runtime_value() {
    let t = tree();
    let mut b =
        fixture::load_bundle(t.path(), Path::new("fixtures/prototype/manifest.json")).unwrap();
    edit_expected(&mut b, "PX03-aggregate-proof-v1", |v| {
        v["proof"]["variance"]["unit"] = json!("not_seconds_squared")
    });
    let v = scenario::verify_metric_scenario(
        &b,
        MetricScenario::AggregateEvidence,
        "PX03-aggregate-proof-v1",
    )
    .unwrap();
    assert!(!v.passed);
    assert_eq!(v.actual.unwrap().results[0].state, QualityState::Unknown);
}
