use aether_poc::{
    enrichment_data, enrichment_fixture as ef, enrichment_scenario as es, fixture,
    identity::*,
    observation::{self, QualityState},
};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn bundle() -> fixture::ValidatedBundle {
    fixture::load_bundle(&root(), Path::new("fixtures/prototype/manifest.json")).unwrap()
}
fn extension() -> ef::EnrichmentFixtureBundle {
    ef::load_enrichment_bundle(&root()).unwrap()
}
fn key(name: &str) -> EntityKey {
    EntityKey {
        scope: "fixture".into(),
        kind: "process".into(),
        locator: name.into(),
        incarnation: "one".into(),
        host: None,
        boot: None,
        namespace: None,
    }
}
fn node(id: &str, k: &EntityKey) -> MetadataRecord {
    MetadataRecord::Node {
        node: EntityNode {
            id: id.into(),
            key: k.clone(),
            validity: Some(Validity {
                start_us: 0,
                end_us: 20,
                clock: "fixture".into(),
                uncertainty_us: 0,
            }),
            evidence: vec![id.into()],
        },
    }
}
fn rel(id: &str, k: &EntityKey, targets: &[&str]) -> MetadataRecord {
    MetadataRecord::Relation {
        relation: Relationship {
            id: id.into(),
            from: k.clone(),
            to: targets
                .iter()
                .map(|t| EntityRef::Symbolic {
                    scope: k.scope.clone(),
                    locator: (*t).into(),
                })
                .collect(),
            kind: "container".into(),
            validity: Some(Validity {
                start_us: 0,
                end_us: 10,
                clock: "fixture".into(),
                uncertainty_us: 0,
            }),
            complete: true,
            evidence: vec![id.into()],
            denied: false,
        },
    }
}
fn query(k: &EntityKey, time: u64) -> AssociationQuery {
    AssociationQuery {
        id: "q".into(),
        from: k.clone(),
        relation: "container".into(),
        selector: TimeSelector::At {
            time_us: time,
            clock: "fixture".into(),
            uncertainty_us: 0,
        },
        is_set: false,
    }
}
fn measurement(k: &EntityKey) -> EnrichmentInputMeasurement {
    EnrichmentInputMeasurement {
        id: "measurement".into(),
        native: json!({"name":"raw_counter","type":"counter","labels":{"model":"same"}}),
        emitter: EntityRef::Key { key: k.clone() },
        subject: Some(EntityRef::Key { key: k.clone() }),
        value: json!(100),
        unit: "requests".into(),
        scope: "replica".into(),
    }
}
fn tree() -> tempfile::TempDir {
    let t = tempfile::tempdir().unwrap();
    let src = root();
    let m: Value =
        serde_json::from_slice(&fs::read(src.join("evidence/manifest-v1.json")).unwrap()).unwrap();
    let mut files: Vec<String> = m["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["path"].as_str().unwrap().into())
        .collect();
    files.extend(
        [
            "evidence/manifest-v1.json",
            "evidence/manifest-v1.sha256",
            "metadata-enrichment-matrix.md",
            "docs/prototype-metric-execution-status.json",
            "evidence/prototype/milestone-4/artifacts.sha256",
        ]
        .map(str::to_string),
    );
    for file in files {
        let target = t.path().join(&file);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(src.join(file), target).unwrap();
    }
    aether_validation::generate_records(t.path()).unwrap();
    aether_validation::author_fixtures(t.path()).unwrap();
    enrichment_data::author_enrichment_fixtures(t.path()).unwrap();
    t
}
fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_aether-poc"))
        .current_dir(root())
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn original_enrichment_variants_match_all_seventeen_assertions() {
    let b = bundle();
    let mut checks = 0;
    let mut new = 0;
    for group in &b.manifest.fixtures {
        if let Some(s) = es::EnrichmentScenario::for_group(&group.experiment_id) {
            for variant in &group.variants {
                let report = es::verify_enrichment_scenario(&b, s, &variant.id).unwrap();
                assert!(report.passed, "{}", variant.id);
                checks += report.checks.len();
                new += report
                    .checks
                    .iter()
                    .filter(|c| c.newly_exercised_in_m5)
                    .count();
            }
        }
    }
    assert_eq!((checks, new), (17, 7));
}
#[test]
fn all_seven_boundary_cases_have_independent_matching_answers() {
    let e = extension();
    let mut checks = 0;
    for id in e.inputs.keys() {
        let v = es::verify_boundary_case(&e, id).unwrap();
        assert!(v.passed, "{id}: {:?}", v.checks);
        checks += v.checks.len();
    }
    assert_eq!(checks, 69);
}
#[test]
fn half_open_lifetimes_and_crossing_intervals_are_enforced() {
    let r = es::run_boundary_case(&extension(), "E5-01").unwrap();
    assert_eq!(r.queries["old-at-5"].candidates[0].locator(), "container-A");
    for id in ["old-at-10", "old-at-20", "new-at-20", "crossing"] {
        assert_eq!(r.queries[id].state, QualityState::Unknown);
    }
    for id in ["new-at-10", "new-at-15"] {
        assert_eq!(r.queries[id].candidates[0].locator(), "container-B");
    }
}
#[test]
fn no_first_candidate_wins_and_reported_candidates_are_untimed() {
    let r = es::run_enrichment_scenario(
        &bundle(),
        es::EnrichmentScenario::PublisherIdentity,
        "PX04-ambiguous-publisher-v1",
    )
    .unwrap();
    assert_eq!(r.results[0].value, json!(["p1", "p2"]));
    assert_eq!(r.associations["publisher"].state, QualityState::Ambiguous);
    assert_eq!(
        r.associations["publisher"].temporal_status,
        "unspecified_input"
    );
    assert!(r.permutation_invariant);
}
#[test]
fn remote_subject_stays_distinct_from_frontend_host() {
    let r = es::run_enrichment_scenario(
        &bundle(),
        es::EnrichmentScenario::PublisherIdentity,
        "PX04-overlap-and-remote-v1",
    )
    .unwrap();
    assert_eq!(
        r.results
            .iter()
            .find(|r| r.target == "selected_boundary")
            .unwrap()
            .value,
        10
    );
    assert!(
        r.results
            .iter()
            .find(|r| r.target == "worker_host")
            .unwrap()
            .value
            .is_null()
    );
    let fact = r
        .context_facts
        .iter()
        .find(|f| f["subject"] == "remote-worker")
        .unwrap();
    assert_eq!(fact["emitter"], "frontend");
    assert!(fact["subject_host"].is_null());
}
#[test]
fn historical_and_new_container_owners_use_distinct_incarnations() {
    let r = es::run_enrichment_scenario(
        &bundle(),
        es::EnrichmentScenario::ProcessLifecycle,
        "PX06-pid-reuse-v1",
    )
    .unwrap();
    assert_eq!(
        r.results
            .iter()
            .find(|r| r.target == "historical_owner")
            .unwrap()
            .value,
        "container-A"
    );
    assert_eq!(
        r.results
            .iter()
            .find(|r| r.target == "new_owner")
            .unwrap()
            .value,
        "container-B"
    );
    assert_eq!(
        r.associations["historical_owner"].state,
        QualityState::Resolved
    );
    assert!(
        r.associations["historical_owner"]
            .evidence
            .contains(&"old-membership".into())
    );
    assert_ne!(
        r.associations["historical_owner"].from.incarnation,
        r.associations["new_owner"].from.incarnation
    );
}
#[test]
fn denied_runtime_preserves_process_context_without_guessing_container() {
    let r = es::run_enrichment_scenario(
        &bundle(),
        es::EnrichmentScenario::ProcessLifecycle,
        "PX06-runtime-denied-v1",
    )
    .unwrap();
    assert_eq!(r.results[0].state, QualityState::Unknown);
    assert_eq!(
        r.associations["container"].source_visibility,
        ["permission_denied"]
    );
    assert_eq!(
        r.context_facts[0]["known_processes"][0]["incarnation"],
        "start-two"
    );
}
#[test]
fn replica_set_does_not_multiply_measurements_or_infer_participation() {
    let r = es::run_enrichment_scenario(
        &bundle(),
        es::EnrichmentScenario::ResourceMembership,
        "PX05-replica-set-v1",
    )
    .unwrap();
    assert_eq!(r.measurement_count, 1);
    assert_eq!(r.observations[0].measurement.value, 100);
    assert_eq!(r.associations["allocated_resources"].candidates.len(), 2);
    assert_eq!(
        r.associations["allocated_resources"].temporal_status,
        "unspecified_input"
    );
    assert_eq!(
        r.associations["observed_participants"].state,
        QualityState::Unknown
    );
    assert!(matches!(
        r.observations[0].measurement.emitter,
        EntityRef::Unknown { .. }
    ));
}
#[test]
fn partial_membership_and_missing_capacity_remain_qualified() {
    let r = es::run_enrichment_scenario(
        &bundle(),
        es::EnrichmentScenario::ResourceMembership,
        "PX05-partial-membership-v1",
    )
    .unwrap();
    assert_eq!(
        r.associations["allocated_resources"].state,
        QualityState::Partial
    );
    assert_eq!(
        r.results
            .iter()
            .find(|r| r.target == "composition")
            .unwrap()
            .value,
        json!(["gpu-A"])
    );
    assert!(
        r.results
            .iter()
            .find(|r| r.target == "gpu_occupancy")
            .unwrap()
            .value
            .is_null()
    );
    assert_eq!(r.measurement_count, 0);
}
#[test]
fn nic_capability_and_preference_are_not_joined_to_an_active_path() {
    let r = es::run_enrichment_scenario(
        &bundle(),
        es::EnrichmentScenario::FabricContext,
        "PX07-inventory-only-v1",
    )
    .unwrap();
    assert_eq!(
        r.results
            .iter()
            .find(|r| r.target == "capabilities")
            .unwrap()
            .state,
        QualityState::ReportedOnly
    );
    assert_eq!(
        r.associations["active_transport"].state,
        QualityState::Unknown
    );
    let cap = r
        .context_facts
        .iter()
        .find(|f| f["reported_model"] == "symbolic-NIC")
        .unwrap();
    assert!(cap["nic_foreign_key"].is_null());
    assert!(cap["loaded_cuda_nccl"].is_null());
    assert_eq!(cap["configured_preference"], "roce");
    assert!(r.context_facts.iter().any(|f| f["value"] == 1024));
}
#[test]
fn tcp_control_peer_never_proves_collective_payload_transport() {
    let r = es::run_enrichment_scenario(
        &bundle(),
        es::EnrichmentScenario::FabricContext,
        "PX07-control-socket-only-v1",
    )
    .unwrap();
    assert_eq!(r.results[0].target, "operation_to_nic");
    assert_eq!(r.results[0].state, QualityState::Unknown);
    assert_eq!(r.context_facts[0]["protocol"], "tcp");
    assert!(r.context_facts[0]["resolved_peer"].is_null());
}
#[test]
fn boot_namespace_and_missing_incarnation_queries_do_not_merge() {
    let r = es::run_boundary_case(&extension(), "E5-03").unwrap();
    assert_ne!(
        r.queries["boot-A"].candidates,
        r.queries["boot-B"].candidates
    );
    assert_eq!(
        r.queries["missing-incarnation"].state,
        QualityState::Unknown
    );
    assert_eq!(r.queries["wrong-namespace"].state, QualityState::Unknown);
    let r = es::run_boundary_case(&extension(), "E5-06").unwrap();
    assert_eq!(r.queries["other-netns"].state, QualityState::Unknown);
}
#[test]
fn new_membership_does_not_relabel_old_observation() {
    let r = es::run_boundary_case(&extension(), "E5-05").unwrap();
    assert_eq!(r.queries["old"], r.queries["old-after"]);
    assert_ne!(
        r.queries["old"].candidates,
        r.queries["new-after"].candidates
    );
    assert_eq!(r.actual.observations[0].measurement.value, 100);
    assert!(r.actual.history_unchanged);
    assert_eq!(
        r.actual.observations[0].relationships["allocated_resources"].candidates[0].locator(),
        "gpu-A"
    );
}
#[test]
fn duplicate_evidence_is_idempotent_and_conflicts_are_not_last_write_wins() {
    let k = key("17");
    let n = node("n", &k);
    let g = IdentityGraph::from_records(&[n.clone(), n.clone()]).unwrap();
    assert_eq!(g.record_count(), 1);
    assert_eq!(
        g.with_records(std::slice::from_ref(&n))
            .unwrap()
            .record_count(),
        1
    );
    let mut bad = n;
    if let MetadataRecord::Node { node } = &mut bad {
        node.validity.as_mut().unwrap().end_us = 19;
    }
    assert_eq!(
        g.with_records(&[bad]).unwrap_err().code,
        "identity_conflict"
    );
    let r = es::run_boundary_case(&extension(), "E5-07").unwrap();
    assert_eq!(r.actual.update_error.as_deref(), Some("identity_conflict"));
    assert!(r.actual.history_unchanged);
}
#[test]
fn ambiguous_distinct_evidence_remains_ambiguous_in_any_input_order() {
    let k = key("17");
    let records = vec![node("node", &k), rel("A", &k, &["A"]), rel("B", &k, &["B"])];
    let g = IdentityGraph::from_records(&records).unwrap();
    let mut reverse = records;
    reverse.reverse();
    let other = IdentityGraph::from_records(&reverse).unwrap();
    let q = query(&k, 5);
    assert_eq!(resolve_association(&g, &q), resolve_association(&other, &q));
    assert_eq!(resolve_association(&g, &q).state, QualityState::Ambiguous);
}
#[test]
fn timed_queries_require_compatible_explicit_clocks_and_validity() {
    let k = key("17");
    let mut r = rel("r", &k, &["A"]);
    let node = node("n", &k);
    let g = IdentityGraph::from_records(&[node.clone(), r.clone()]).unwrap();
    for selector in [
        TimeSelector::At {
            time_us: 5,
            clock: "remote".into(),
            uncertainty_us: 0,
        },
        TimeSelector::At {
            time_us: 5,
            clock: "fixture".into(),
            uncertainty_us: 1,
        },
        TimeSelector::During {
            start_us: 5,
            end_us: 15,
            clock: "fixture".into(),
            uncertainty_us: 0,
        },
    ] {
        let mut q = query(&k, 5);
        q.selector = selector;
        assert_eq!(resolve_association(&g, &q).state, QualityState::Unknown);
    }
    if let MetadataRecord::Relation { relation } = &mut r {
        relation.validity = None;
    }
    let g = IdentityGraph::from_records(&[node, r]).unwrap();
    assert_eq!(
        resolve_association(&g, &query(&k, 5)).state,
        QualityState::Unknown
    );
    let mut q = query(&k, 5);
    q.selector = TimeSelector::Unspecified;
    assert_eq!(
        resolve_association(&g, &q).state,
        QualityState::ReportedOnly
    );
}
#[test]
fn dangling_references_and_cross_scope_targets_are_rejected() {
    let k = key("17");
    let mut r = rel("r", &k, &["A"]);
    if let MetadataRecord::Relation { relation } = &mut r {
        relation.to = vec![EntityRef::Key { key: key("absent") }];
    }
    assert!(IdentityGraph::from_records(&[node("n", &k), r.clone()]).is_err());
    if let MetadataRecord::Relation { relation } = &mut r {
        relation.to = vec![EntityRef::Symbolic {
            scope: "other".into(),
            locator: "A".into(),
        }];
    }
    assert!(IdentityGraph::from_records(&[node("n", &k), r]).is_err());
}
#[test]
fn keyed_targets_must_cover_the_query_time() {
    let k = key("17");
    let target = key("container");
    let mut n = node("target", &target);
    if let MetadataRecord::Node { node } = &mut n {
        node.validity.as_mut().unwrap().start_us = 10;
    }
    let mut r = rel("r", &k, &[]);
    if let MetadataRecord::Relation { relation } = &mut r {
        relation.to = vec![EntityRef::Key { key: target }];
    }
    let g = IdentityGraph::from_records(&[node("n", &k), n, r]).unwrap();
    assert_eq!(
        resolve_association(&g, &query(&k, 5)).state,
        QualityState::Unknown
    );
}
#[test]
fn unrelated_measurement_cannot_receive_another_subjects_relationship() {
    let k = key("17");
    let g = IdentityGraph::from_records(&[node("n", &k), rel("r", &k, &["A"])]).unwrap();
    assert_eq!(
        associate_observation(&measurement(&key("unrelated")), &g, &query(&k, 5))
            .unwrap_err()
            .code,
        "observation_query_mismatch"
    );
    let one = associate_observation(&measurement(&k), &g, &query(&k, 5)).unwrap();
    let two = associate_observation(&one.measurement, &g, &query(&k, 5)).unwrap();
    assert_eq!(one, two);
    assert_eq!(
        one.measurement.native,
        json!({"name":"raw_counter","type":"counter","labels":{"model":"same"}})
    );
}
#[test]
fn metadata_validation_rejects_invalid_bounds_identity_and_limits() {
    let k = key("17");
    let mut invalid = node("n", &k);
    if let MetadataRecord::Node { node } = &mut invalid {
        node.validity.as_mut().unwrap().end_us = 0;
    }
    assert!(IdentityGraph::from_records(&[invalid]).is_err());
    let mut missing = k.clone();
    missing.incarnation.clear();
    assert!(IdentityGraph::from_records(&[node("n", &missing)]).is_err());
    let records: Vec<_> = (0..33)
        .map(|i| node(&format!("n{i}"), &key(&format!("p{i}"))))
        .collect();
    assert!(IdentityGraph::from_records(&records).is_err());
}
#[test]
fn extension_authoring_is_repeatable_and_protects_manual_changes() {
    let t = tree();
    assert_eq!(
        enrichment_data::author_enrichment_fixtures(t.path()).unwrap(),
        0
    );
    fs::write(
        t.path().join("fixtures/enrichment-v1/input/E5-01.json"),
        "manual change",
    )
    .unwrap();
    assert_eq!(
        enrichment_data::author_enrichment_fixtures(t.path())
            .unwrap_err()
            .code,
        "authoring_conflict"
    );
}
fn edit_manifest(root: &Path, change: impl FnOnce(&mut Value)) {
    let path = root.join("fixtures/enrichment-v1/manifest.json");
    let mut value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    change(&mut value);
    fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
}
#[test]
fn extension_hashes_missing_files_and_unindexed_files_fail() {
    for mode in 0..3 {
        let t = tree();
        let path = t.path().join("fixtures/enrichment-v1/input/E5-01.json");
        match mode {
            0 => fs::write(path, "tampered").unwrap(),
            1 => fs::remove_file(path).unwrap(),
            _ => fs::write(
                t.path().join("fixtures/enrichment-v1/unindexed.txt"),
                "extra",
            )
            .unwrap(),
        };
        assert!(ef::load_enrichment_bundle(t.path()).is_err());
    }
}
#[test]
fn extension_baseline_identity_duplicate_cases_and_unsafe_paths_fail() {
    for mode in 0..4 {
        let t = tree();
        edit_manifest(t.path(), |m| match mode {
            0 => m["original_manifest_sha256"] = json!("wrong"),
            1 => m["cases"][1]["case_id"] = json!("E5-01"),
            2 => m["cases"][0]["input"]["path"] = json!("../outside.json"),
            _ => m["cases"][0]["input"]["path"] = json!("/tmp/outside.json"),
        });
        assert!(ef::load_enrichment_bundle(t.path()).is_err());
    }
}
#[test]
fn extension_oversized_truncated_and_unknown_schema_fail() {
    for bytes in [
        vec![b' '; 1024 * 1024 + 1],
        b"{".to_vec(),
        b"{\"schema_version\":2}".to_vec(),
    ] {
        let t = tree();
        fs::write(t.path().join("fixtures/enrichment-v1/manifest.json"), bytes).unwrap();
        assert!(ef::load_enrichment_bundle(t.path()).is_err());
    }
}
#[cfg(unix)]
#[test]
fn extension_symlink_is_rejected_even_with_matching_bytes() {
    let t = tree();
    let original = t.path().join("fixtures/enrichment-v1/input/E5-01.json");
    let saved = t.path().join("saved.json");
    fs::rename(&original, &saved).unwrap();
    std::os::unix::fs::symlink(saved, original).unwrap();
    assert!(ef::load_enrichment_bundle(t.path()).is_err());
}
#[test]
fn independent_oracle_mismatch_does_not_change_actual_resolution() {
    let t = tree();
    let mut e = ef::load_enrichment_bundle(t.path()).unwrap();
    let case = e
        .manifest
        .cases
        .iter_mut()
        .find(|c| c.case_id == "E5-01")
        .unwrap();
    let p = e.root.join(&case.expected.path);
    let mut value: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    value["queries"]["old-at-5"]["candidates"] = json!(["wrong-owner"]);
    let bytes = serde_json::to_vec(&value).unwrap();
    case.expected.bytes = bytes.len() as u64;
    case.expected.sha256 = aether_validation::sha256(&bytes);
    fs::write(p, bytes).unwrap();
    let v = es::verify_boundary_case(&e, "E5-01").unwrap();
    assert!(!v.passed);
    assert_eq!(
        v.actual.queries["old-at-5"].candidates[0].locator(),
        "container-A"
    );
}
#[test]
fn oracle_tampering_is_not_read_by_runtime_and_fails_integrity() {
    let t = tree();
    let e = ef::load_enrichment_bundle(t.path()).unwrap();
    fs::write(e.root.join("expected/E5-01.json"), "{}").unwrap();
    assert_eq!(
        es::run_boundary_case(&e, "E5-01").unwrap().queries["old-at-5"].candidates[0].locator(),
        "container-A"
    );
    assert!(es::verify_boundary_case(&e, "E5-01").is_err());
}
#[test]
fn invalid_boundary_query_and_unknown_record_fields_are_rejected() {
    let mut v: Value = serde_json::from_slice(
        &fs::read(root().join("fixtures/enrichment-v1/input/E5-01.json")).unwrap(),
    )
    .unwrap();
    v["queries"][0]["selector"]["clock"] = json!("");
    let input: ef::BoundaryInput = serde_json::from_value(v.clone()).unwrap();
    assert!(ef::validate_boundary_input(&input).is_err());
    v["extra_unknown_field"] = true.into();
    assert!(serde_json::from_value::<ef::BoundaryInput>(v).is_err());
}
#[test]
fn m5_accounting_keeps_original_and_extension_totals_distinct() {
    let summary = es::verify_enrichment_milestone(&bundle(), &extension()).unwrap();
    assert_eq!(summary["original_assertions_indexed"], 49);
    assert_eq!(summary["current_original_matches"], 17);
    assert_eq!(summary["new_original_matches"], 7);
    assert_eq!(summary["prior_matches_rechecked"], 10);
    assert_eq!(
        summary["combined_original_assertion_outcomes"],
        json!({"matched":31,"expected_rejection":2,"proof_verified":1,"deferred":1,"not_run_later_stage":14})
    );
    assert_eq!(summary["groups"].as_array().unwrap().len(), 12);
    assert_eq!(summary["boundary_cases"].as_array().unwrap().len(), 7);
    assert_eq!(summary["runtime_acceptance_readiness"], false);
    assert_eq!(summary["functional_checks_passed"], true);
}
#[test]
fn changed_m4_ledger_cannot_be_imported_as_prior_evidence() {
    let t = tree();
    let b = fixture::load_bundle(t.path(), Path::new("fixtures/prototype/manifest.json")).unwrap();
    let e = ef::load_enrichment_bundle(t.path()).unwrap();
    let path = t.path().join("docs/prototype-metric-execution-status.json");
    let mut v: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    v["functional_checks_passed"] = false.into();
    fs::write(path, serde_json::to_vec(&v).unwrap()).unwrap();
    assert_eq!(
        es::verify_enrichment_milestone(&b, &e).unwrap_err().code,
        "predecessor_invalid"
    );
}
#[test]
fn privacy_and_policy_paths_remain_safe_and_blocked() {
    for id in [
        "PX11-visibility-and-redaction-v1",
        "PX11-forbidden-actions-v1",
    ] {
        let r = es::run_enrichment_scenario(
            &bundle(),
            es::EnrichmentScenario::VisibilityEnrichment,
            id,
        )
        .unwrap();
        assert!(
            !serde_json::to_string(&r)
                .unwrap()
                .contains(observation::MARKER)
        );
        assert!(!r.runtime_acceptance_readiness);
    }
    let r = es::run_boundary_case(&extension(), "E5-07").unwrap();
    assert!(
        !serde_json::to_string(&r)
            .unwrap()
            .contains(observation::MARKER)
    );
    assert_eq!(
        r.actual.observations[0].measurement.native["fixture"],
        "safe"
    );
    assert_eq!(r.queries["denied"].source_visibility, ["permission_denied"]);
}
#[test]
fn command_pairing_errors_and_summary_determinism() {
    for args in [
        vec![
            "run-scenario",
            "--scenario",
            "publisher-identity",
            "--variant",
            "PX07-inventory-only-v1",
        ],
        vec![
            "run-scenario",
            "--scenario",
            "enrichment-boundaries",
            "--case",
            "E5-99",
        ],
        vec![
            "run-scenario",
            "--scenario",
            "enrichment-boundaries",
            "--case",
            "E5-01",
            "--variant",
            "PX06-pid-reuse-v1",
        ],
        vec![
            "run-scenario",
            "--scenario",
            "publisher-identity",
            "--case",
            "E5-01",
            "--variant",
            "PX04-ambiguous-publisher-v1",
        ],
    ] {
        let o = cli(&args);
        assert!(!o.status.success());
        assert!(o.stdout.is_empty());
    }
    let a = cli(&["verify-milestone", "--milestone", "5"]);
    let b = cli(&["verify-milestone", "--milestone", "5"]);
    assert!(a.status.success(), "{}", String::from_utf8_lossy(&a.stderr));
    assert!(b.status.success());
    assert_eq!(a.stdout, b.stdout);
    assert!(!String::from_utf8_lossy(&a.stdout).contains(observation::MARKER));
}
