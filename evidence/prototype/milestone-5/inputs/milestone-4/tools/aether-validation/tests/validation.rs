use aether_validation::{
    check_evidence, check_traceability, generate_records, parse_association_rows, sha256,
};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::TempDir;
fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn load(root: &Path, path: &str) -> Value {
    serde_json::from_slice(&fs::read(root.join(path)).unwrap()).unwrap()
}
fn save(root: &Path, path: &str, v: &Value) {
    let mut bytes = serde_json::to_vec_pretty(v).unwrap();
    bytes.push(b'\n');
    fs::write(root.join(path), bytes).unwrap();
}
fn tree(planning: bool) -> TempDir {
    let temp = tempfile::tempdir().unwrap();
    let source = workspace();
    let manifest = load(&source, "evidence/manifest-v1.json");
    let mut files: Vec<String> = manifest["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["path"].as_str().unwrap().to_owned())
        .collect();
    files.extend([
        "evidence/manifest-v1.json".into(),
        "evidence/manifest-v1.sha256".into(),
    ]);
    for file in files {
        let p = temp.path().join(&file);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::copy(source.join(file), p).unwrap();
    }
    if planning {
        generate_records(temp.path()).unwrap();
    }
    temp
}
fn rejection(root: &Path, path: &str, change: impl FnOnce(&mut Value), reason: &str) {
    let mut v = load(root, path);
    change(&mut v);
    save(root, path, &v);
    let errors = check_traceability(root).expect_err("corrupt planning record accepted");
    let message = errors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        message.contains(reason),
        "wrong rejection: {message}; expected {reason}"
    );
}
fn refresh_test_artifact(root: &Path, path: &str) {
    // Only temporary test trees: exercise semantic checks past the hash boundary.
    let mut m = load(root, "evidence/manifest-v1.json");
    let bytes = fs::read(root.join(path)).unwrap();
    let item = m["artifacts"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["path"] == path)
        .unwrap();
    item["bytes"] = json!(bytes.len());
    item["sha256"] = json!(sha256(&bytes));
    save(root, "evidence/manifest-v1.json", &m);
    fs::write(
        root.join("evidence/manifest-v1.sha256"),
        format!(
            "{}  manifest-v1.json\n",
            sha256(&fs::read(root.join("evidence/manifest-v1.json")).unwrap())
        ),
    )
    .unwrap();
}
#[test]
fn evidence_matches_frozen_bundle() {
    let t = tree(false);
    let s = check_evidence(t.path()).unwrap();
    assert_eq!((s.sources, s.artifacts, s.historical_u), (10, 24, 26));
}
#[test]
fn corrupt_snapshot_rejected() {
    let t = tree(false);
    fs::write(
        t.path().join("evidence/inputs/v1/E06.source.txt"),
        "corrupt",
    )
    .unwrap();
    assert!(
        check_evidence(t.path()).unwrap_err()[0]
            .reason
            .contains("checksum mismatch")
    );
}
#[test]
fn corrupt_manifest_rejected() {
    let t = tree(false);
    let p = t.path().join("evidence/manifest-v1.json");
    let mut bytes = fs::read(&p).unwrap();
    bytes.push(b' ');
    fs::write(p, bytes).unwrap();
    assert!(
        check_evidence(t.path()).unwrap_err()[0]
            .reason
            .contains("manifest SHA-256 mismatch")
    );
}
#[test]
fn missing_snapshot_rejected() {
    let t = tree(false);
    fs::remove_file(t.path().join("evidence/inputs/v1/E06.source.txt")).unwrap();
    assert!(check_evidence(t.path()).is_err());
}
#[test]
fn rehashed_false_deployment_claim_rejected() {
    let t = tree(false);
    let p = "validation/first-environment-v1.json";
    let mut v = load(t.path(), p);
    v["readiness"]["real_deployment_available"] = json!(true);
    save(t.path(), p, &v);
    refresh_test_artifact(t.path(), p);
    assert!(
        check_evidence(t.path()).unwrap_err()[0]
            .reason
            .contains("environment cannot claim")
    );
}
#[test]
fn rehashed_index_disagreement_rejected() {
    let t = tree(false);
    let p = "evidence/inputs/v1/source-metadata.json";
    let mut v = load(t.path(), p);
    v["entries"][0]["commit"] = json!("wrong");
    save(t.path(), p, &v);
    refresh_test_artifact(t.path(), p);
    assert!(
        check_evidence(t.path()).unwrap_err()[0]
            .reason
            .contains("source index")
    );
}
#[test]
fn all_requirements_tracked_without_forcing_all_selected() {
    let t = tree(true);
    let s = check_traceability(t.path()).unwrap();
    assert_eq!((s.metrics, s.associations, s.experiments), (250, 58, 12));
    assert!(s.selected_metrics > 0 && s.selected_metrics < 250);
    let v = load(t.path(), "docs/prototype-status.json");
    assert!(v["metrics"].as_array().unwrap().iter().any(|r|r["selected_for_fixture"]==false && r["prototype_state"]=="not_prototyped"));
}
#[test]
fn missing_metric_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-status.json",
        |v| {
            v["metrics"].as_array_mut().unwrap().remove(0);
        },
        "missing metric L1-001",
    );
}
#[test]
fn duplicate_association_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-status.json",
        |v| {
            let a = v["associations"][0].clone();
            v["associations"].as_array_mut().unwrap().push(a);
        },
        "duplicate ID M01",
    );
}
#[test]
fn changed_definition_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-status.json",
        |v| {
            v["metrics"][0]["baseline_requirement"]["definition"] = json!("changed");
        },
        "changed baseline definition L1-001",
    );
}
#[test]
fn changed_route_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-status.json",
        |v| {
            v["metrics"][0]["research_mapping"]["route"] = json!("E");
        },
        "changed research route/dependency L1-001",
    );
}
#[test]
fn changed_association_definition_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-status.json",
        |v| {
            v["associations"][0]["baseline_association"]["cells"][4] = json!("no evidence needed");
        },
        "changed association definition M01",
    );
}
#[test]
fn bogus_experiment_link_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-status.json",
        |v| {
            v["metrics"][0]["experiment_ids"] = json!(["PX99"]);
        },
        "selection links disagree",
    );
}
#[test]
fn bogus_source_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-experiments.json",
        |v| {
            v["experiments"][0]["pinned_source_ids"] = json!(["E99"]);
        },
        "unknown pinned source ID",
    );
}
#[test]
fn dropped_metadata_dependency_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-experiments.json",
        |v| {
            v["experiments"][0]["association_ids"]
                .as_array_mut()
                .unwrap()
                .retain(|a| a != "M58");
        },
        "dropped metadata dependency",
    );
}
#[test]
fn broken_budget_link_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-experiments.json",
        |v| {
            v["experiments"][0]["budget_record"] = json!("../outside.json");
        },
        "broken budget link",
    );
}
#[test]
fn false_synthetic_execution_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-status.json",
        |v| {
            v["metrics"][0]["synthetic_execution_status"] = json!("passed");
        },
        "synthetic_execution_status must remain not_run",
    );
}
#[test]
fn false_runtime_readiness_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-budgets/PX11.json",
        |v| {
            v["readiness"]["ready_for_acceptance_run"] = json!(true);
        },
        "acceptance readiness contradicts",
    );
}
#[test]
fn invented_agreement_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-budgets/PX01.json",
        |v| {
            v["agreement"]["product_owner"] = json!("invented owner");
        },
        "agreement provenance",
    );
}
#[test]
fn false_real_verification_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-status.json",
        |v| {
            v["metrics"][0]["capability_verification"] = json!("verified");
        },
        "capability_verification must remain not_run",
    );
}
#[test]
fn invalid_execution_milestone_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-experiments.json",
        |v| {
            v["experiments"][0]["execution_milestones"] = json!([1]);
        },
        "execution milestones",
    );
}
#[test]
fn missing_criterion_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-budgets/PX01.json",
        |v| {
            v["acceptance_criteria"].as_array_mut().unwrap().pop();
        },
        "criterion IDs changed",
    );
}
#[test]
fn invalid_safety_duration_rejected() {
    let t = tree(true);
    rejection(
        t.path(),
        "docs/prototype-budgets/PX01.json",
        |v| {
            v["finite_safety_limits"]["max_run_duration"]["value"] = json!(-1);
        },
        "invalid finite positive limit max_run_duration",
    );
}
#[test]
fn generator_is_byte_stable_and_preserves_manual_edits() {
    let t = tree(true);
    assert_eq!(generate_records(t.path()).unwrap(), 0);
    let p = t.path().join("docs/prototype-scope.md");
    fs::write(&p, "manual notes\n").unwrap();
    assert!(
        generate_records(t.path()).unwrap_err()[0]
            .reason
            .contains("refusing to overwrite")
    );
    assert_eq!(fs::read_to_string(p).unwrap(), "manual notes\n");
}
#[test]
fn malformed_json_is_failure_not_panic() {
    let t = tree(true);
    fs::write(t.path().join("docs/prototype-status.json"), "{").unwrap();
    assert!(check_traceability(t.path()).is_err());
}
#[test]
fn missing_status_is_failure_not_empty_success() {
    let t = tree(false);
    assert!(check_traceability(t.path()).is_err());
}
#[test]
fn cli_uses_requested_root_and_nonzero_exit() {
    let t = tree(true);
    let bin = env!("CARGO_BIN_EXE_aether-validate");
    assert!(
        Command::new(bin)
            .args(["traceability", "--root"])
            .arg(t.path())
            .output()
            .unwrap()
            .status
            .success()
    );
    let bad = Command::new(bin).arg("unknown").output().unwrap();
    assert_eq!(bad.status.code(), Some(2));
}
#[test]
fn metadata_parser_handles_escaped_pipe_and_rejects_duplicates() {
    let source = fs::read_to_string(workspace().join("metadata-enrichment-matrix.md")).unwrap();
    let escaped = source.replacen(
        "Environment / cluster identity",
        "Environment \\| cluster identity",
        1,
    );
    let rows = parse_association_rows(&escaped).unwrap();
    assert_eq!(rows.len(), 58);
    assert!(rows[0]["cells"][1].as_str().unwrap().contains("\\|"));
    let duplicate = source.lines().find(|l| l.starts_with("| M01 |")).unwrap();
    assert!(parse_association_rows(&format!("{source}\n{duplicate}")).is_err());
}
#[cfg(unix)]
#[test]
fn input_symlink_escape_rejected_even_with_matching_content() {
    use std::os::unix::fs::symlink;
    let t = tree(false);
    let outside = tempfile::tempdir().unwrap();
    let rel = "evidence/inputs/v1/E06.source.txt";
    let target = outside.path().join("source.txt");
    fs::copy(t.path().join(rel), &target).unwrap();
    fs::remove_file(t.path().join(rel)).unwrap();
    symlink(target, t.path().join(rel)).unwrap();
    assert!(
        check_evidence(t.path()).unwrap_err()[0]
            .reason
            .contains("escapes root")
    );
}
#[cfg(unix)]
#[test]
fn generator_cannot_write_through_external_docs_symlink() {
    use std::os::unix::fs::symlink;
    let t = tree(false);
    let outside = tempfile::tempdir().unwrap();
    symlink(outside.path(), t.path().join("docs")).unwrap();
    assert!(
        generate_records(t.path()).unwrap_err()[0]
            .reason
            .contains("output parent escapes")
    );
    assert!(fs::read_dir(outside.path()).unwrap().next().is_none());
}
