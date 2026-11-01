use aether_validation::{author_fixtures, check_fixtures, generate_records, sha256};
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
fn tree() -> TempDir {
    let t = tempfile::tempdir().unwrap();
    let source = workspace();
    let m = load(&source, "evidence/manifest-v1.json");
    let mut files: Vec<String> = m["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["path"].as_str().unwrap().into())
        .collect();
    files.extend([
        "evidence/manifest-v1.json".into(),
        "evidence/manifest-v1.sha256".into(),
    ]);
    for file in files {
        let target = t.path().join(&file);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(source.join(file), target).unwrap();
    }
    generate_records(t.path()).unwrap();
    author_fixtures(t.path()).unwrap();
    t
}
fn rehash_ledger(root: &Path) {
    let mut l = load(root, "docs/prototype-fixture-status.json");
    l["manifest_sha256"] = json!(sha256(
        &fs::read(root.join("fixtures/prototype/manifest.json")).unwrap()
    ));
    save(root, "docs/prototype-fixture-status.json", &l);
}
fn change_manifest(root: &Path, f: impl FnOnce(&mut Value)) {
    let mut m = load(root, "fixtures/prototype/manifest.json");
    f(&mut m);
    save(root, "fixtures/prototype/manifest.json", &m);
    rehash_ledger(root);
}
fn payload_path(root: &Path, variant: &str, role: &str) -> String {
    let m = load(root, "fixtures/prototype/manifest.json");
    m["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|g| g["variants"].as_array().unwrap())
        .find(|v| v["id"] == variant)
        .unwrap()["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["role"] == role)
        .unwrap()["path"]
        .as_str()
        .unwrap()
        .into()
}
fn rehash_payload(root: &Path, path: &str) {
    let bytes = fs::read(root.join("fixtures/prototype").join(path)).unwrap();
    change_manifest(root, |m| {
        for g in m["fixtures"].as_array_mut().unwrap() {
            for v in g["variants"].as_array_mut().unwrap() {
                for a in v["artifacts"].as_array_mut().unwrap() {
                    if a["path"] == path {
                        a["bytes"] = json!(bytes.len());
                        a["sha256"] = json!(sha256(&bytes));
                    }
                }
            }
        }
    });
}
fn edit_payload(root: &Path, variant: &str, role: &str, f: impl FnOnce(&mut Value)) {
    let path = payload_path(root, variant, role);
    let full = format!("fixtures/prototype/{path}");
    let mut v = load(root, &full);
    f(&mut v);
    save(root, &full, &v);
    rehash_payload(root, &path);
}
fn reject(root: &Path, reason: &str) {
    let errors = check_fixtures(root).expect_err("corrupted fixtures accepted");
    let message = errors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        message.contains(reason),
        "wrong reason: {message}; wanted {reason}"
    );
}
#[test]
fn fixture_bundle_covers_all_groups_and_known_counts() {
    let t = tree();
    let s = check_fixtures(t.path()).unwrap();
    assert_eq!(
        (s.groups, s.variants, s.files, s.invalid_payloads),
        (12, 36, 82, 2)
    );
}
#[test]
fn intentional_invalid_payload_is_really_rejected() {
    let t = tree();
    check_fixtures(t.path()).unwrap();
    let p = payload_path(t.path(), "PX01-malformed-type-v1", "endpoint");
    let full = t.path().join("fixtures/prototype").join(&p);
    let text = fs::read_to_string(&full)
        .unwrap()
        .replace(" invented_type\n", " gauge\n");
    fs::write(full, text).unwrap();
    rehash_payload(t.path(), &p);
    reject(t.path(), "negative payload was not rejected");
}
#[test]
fn wrong_negative_rejection_reason_fails() {
    let t = tree();
    change_manifest(t.path(), |m| {
        m["fixtures"][0]["variants"][7]["expected_payload_error"] = json!("HISTOGRAM_INCONSISTENT");
    });
    reject(t.path(), "negative payload rejected for wrong reason");
}
#[test]
fn corrupt_hash_is_not_a_negative_fixture_exemption() {
    let t = tree();
    let p = payload_path(t.path(), "PX01-malformed-type-v1", "endpoint");
    fs::write(t.path().join("fixtures/prototype").join(p), "bad").unwrap();
    reject(t.path(), "fixture artifact hash/length mismatch");
}
#[test]
fn missing_experiment_group_fails() {
    let t = tree();
    change_manifest(t.path(), |m| {
        m["fixtures"].as_array_mut().unwrap().pop();
    });
    reject(t.path(), "missing experiment groups");
}
#[test]
fn missing_variant_fails() {
    let t = tree();
    change_manifest(t.path(), |m| {
        m["fixtures"][2]["variants"].as_array_mut().unwrap().pop();
    });
    reject(t.path(), "missing or unbounded variants");
}
#[test]
fn missing_oracle_file_fails() {
    let t = tree();
    let p = payload_path(t.path(), "PX02-success-rate-v1", "expected");
    fs::remove_file(t.path().join("fixtures/prototype").join(p)).unwrap();
    assert!(check_fixtures(t.path()).is_err());
}
#[test]
fn dropped_dependency_fails() {
    let t = tree();
    change_manifest(t.path(), |m| {
        m["fixtures"][0]["association_ids"]
            .as_array_mut()
            .unwrap()
            .pop();
    });
    reject(t.path(), "dropped/changed registry links: association_ids");
}
#[test]
fn unknown_source_id_fails() {
    let t = tree();
    change_manifest(t.path(), |m| {
        m["fixtures"][0]["pinned_source_ids"] = json!(["E99"]);
    });
    reject(
        t.path(),
        "dropped/changed registry links: pinned_source_ids",
    );
}
#[test]
fn bad_payload_declaration_cannot_hide_unknown_source() {
    let t = tree();
    edit_payload(t.path(), "PX01-malformed-type-v1", "input", |v| {
        v["records"][0]["source_input_ids"] = json!(["E99"]);
    });
    reject(t.path(), "unknown endpoint source ID");
}
#[test]
fn wrong_budget_hash_fails() {
    let t = tree();
    change_manifest(t.path(), |m| {
        m["fixtures"][0]["budget_sha256"] = json!("bad");
    });
    reject(t.path(), "budget path/hash mismatch");
}
#[test]
fn duplicate_entity_incarnation_fails() {
    let t = tree();
    edit_payload(t.path(), "PX06-pid-reuse-v1", "input", |v| {
        let mut r = v["records"][0].clone();
        r["id"] = json!("different-record-same-entity");
        v["records"].as_array_mut().unwrap().push(r);
    });
    reject(t.path(), "duplicate entity incarnation");
}
#[test]
fn invalid_clock_unit_fails() {
    let t = tree();
    edit_payload(t.path(), "PX08-complete-group-v1", "input", |v| {
        v["clocks"][0]["unit"] = json!("ticks");
    });
    reject(t.path(), "invalid/duplicate clock or units");
}
#[test]
fn negative_clock_uncertainty_fails() {
    let t = tree();
    edit_payload(t.path(), "PX08-complete-group-v1", "input", |v| {
        v["clocks"][0]["uncertainty_us"] = json!(-1);
    });
    reject(t.path(), "expected u64");
}
#[test]
fn unknown_record_clock_fails() {
    let t = tree();
    edit_payload(t.path(), "PX08-complete-group-v1", "input", |v| {
        v["records"][0]["clock_domain"] = json!("nonexistent");
    });
    reject(t.path(), "temporal record requires a declared clock domain");
}
#[test]
fn invalid_rank_timestamp_fails() {
    let t = tree();
    edit_payload(t.path(), "PX08-complete-group-v1", "input", |v| {
        v["records"][0]["completion_us"] = json!(90);
    });
    reject(t.path(), "invalid rank timestamps");
}
#[test]
fn changed_rate_expected_number_fails() {
    let t = tree();
    edit_payload(t.path(), "PX02-success-rate-v1", "expected", |v| {
        v["assertions"][0]["expected_value"] = json!(6);
    });
    reject(t.path(), "expected numeric answer contradicts");
}
#[test]
fn changed_native_expected_values_fail() {
    let t = tree();
    edit_payload(t.path(), "PX01-tgi-v1", "expected", |v| {
        v["assertions"][0]["expected_value"]["sample_values"] = json!([999]);
    });
    reject(t.path(), "native oracle sample values contradict");
}
#[test]
fn changed_counterexample_proof_fails() {
    let t = tree();
    edit_payload(t.path(), "PX03-aggregate-proof-v1", "expected", |v| {
        v["assertions"][0]["expected_value"]["variance_a"] = json!(0);
    });
    reject(t.path(), "proof oracle number inconsistent");
}
#[test]
fn inconsistent_histogram_is_rejected_even_after_rehash() {
    let t = tree();
    let p = payload_path(t.path(), "PX01-vllm-v1", "endpoint");
    let full = t.path().join("fixtures/prototype").join(&p);
    fs::write(
        &full,
        fs::read_to_string(&full).unwrap().replace(
            "} 2\nvllm:iteration_tokens_total_sum",
            "} 3\nvllm:iteration_tokens_total_sum",
        ),
    )
    .unwrap();
    rehash_payload(t.path(), &p);
    reject(t.path(), "HISTOGRAM_INCONSISTENT");
}
#[test]
fn unindexed_file_fails() {
    let t = tree();
    fs::write(t.path().join("fixtures/prototype/events/extra.json"), "{}").unwrap();
    reject(t.path(), "unindexed payload files");
}
#[test]
fn truncated_payload_fails_decode_after_rehash() {
    let t = tree();
    let p = payload_path(t.path(), "PX02-success-rate-v1", "input");
    fs::write(t.path().join("fixtures/prototype").join(&p), "{").unwrap();
    rehash_payload(t.path(), &p);
    reject(t.path(), "EOF while parsing");
}
#[test]
fn oversized_payload_is_bounded_before_parse() {
    let t = tree();
    let p = payload_path(t.path(), "PX02-success-rate-v1", "input");
    fs::write(
        t.path().join("fixtures/prototype").join(&p),
        vec![b' '; 1_048_577],
    )
    .unwrap();
    reject(t.path(), "actual payload exceeds byte limit");
}
#[test]
fn path_traversal_fails() {
    let t = tree();
    change_manifest(t.path(), |m| {
        m["fixtures"][0]["variants"][0]["artifacts"][0]["path"] = json!("../outside.json");
    });
    reject(t.path(), "unsafe relative path");
}
#[test]
fn false_real_provenance_fails() {
    let t = tree();
    edit_payload(t.path(), "PX07-inventory-only-v1", "input", |v| {
        v["synthetic"] = json!(false);
    });
    reject(t.path(), "input must identify synthetic");
}
#[test]
fn false_record_capture_claim_fails() {
    let t = tree();
    edit_payload(t.path(), "PX07-inventory-only-v1", "input", |v| {
        v["records"][0]["captured"] = json!(true);
    });
    reject(t.path(), "cannot claim real/captured provenance");
}
#[test]
fn false_runtime_success_fails() {
    let t = tree();
    edit_payload(t.path(), "PX08-complete-group-v1", "expected", |v| {
        v["experiment_execution_status"] = json!("passed");
    });
    reject(t.path(), "oracle must be synthetic and unexecuted");
}
#[test]
fn false_ledger_readiness_fails() {
    let t = tree();
    let p = "docs/prototype-fixture-status.json";
    let mut l = load(t.path(), p);
    l["runtime_acceptance_readiness"] = json!(true);
    save(t.path(), p, &l);
    reject(t.path(), "ledger cannot claim runtime/deployed readiness");
}
#[test]
fn false_construction_pass_without_evidence_fails() {
    let t = tree();
    let p = "docs/prototype-fixture-status.json";
    let mut l = load(t.path(), p);
    l["entries"][0]["fixture_validation_status"] = json!("passed");
    save(t.path(), p, &l);
    assert!(check_fixtures(t.path()).is_err());
}
#[test]
fn arbitrary_fault_command_fails() {
    let t = tree();
    edit_payload(t.path(), "PX10-protected-replay-v1", "input", |v| {
        v["records"][2]["action"] = json!("execute_shell_command");
    });
    reject(t.path(), "fault is not an allowlisted data action");
}
#[test]
fn ledger_variant_omission_fails() {
    let t = tree();
    let p = "docs/prototype-fixture-status.json";
    let mut l = load(t.path(), p);
    l["entries"][0]["variant_ids"].as_array_mut().unwrap().pop();
    save(t.path(), p, &l);
    reject(t.path(), "ledger variants disagree");
}
#[test]
fn authoring_is_repeatable_and_protects_manual_content() {
    let t = tree();
    assert_eq!(author_fixtures(t.path()).unwrap(), 0);
    let p = t.path().join("fixtures/prototype/README.md");
    fs::write(&p, "manual fixture notes").unwrap();
    assert!(
        author_fixtures(t.path()).unwrap_err()[0]
            .reason
            .contains("refusing to overwrite")
    );
    assert_eq!(fs::read_to_string(p).unwrap(), "manual fixture notes");
}
#[test]
fn fixture_cli_validates_only_the_requested_root() {
    let t = tree();
    let result = Command::new(env!("CARGO_BIN_EXE_aether-validate"))
        .args(["fixtures", "--root"])
        .arg(t.path())
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("synthetic_experiments_executed=0"));
}
#[cfg(unix)]
#[test]
fn matching_content_symlink_escape_fails() {
    use std::os::unix::fs::symlink;
    let t = tree();
    let outside = tempfile::tempdir().unwrap();
    let p = payload_path(t.path(), "PX05-replica-set-v1", "input");
    let full = t.path().join("fixtures/prototype").join(p);
    let target = outside.path().join("payload.json");
    fs::copy(&full, &target).unwrap();
    fs::remove_file(&full).unwrap();
    symlink(target, full).unwrap();
    reject(t.path(), "path escapes root");
}
#[test]
fn sensitive_marker_cannot_be_laundered_into_retained_oracle() {
    let t = tree();
    edit_payload(t.path(), "PX11-visibility-and-redaction-v1", "input", |v| {
        v["records"][0]["retained_labels"] = json!({"fixture":"SYNTHETIC_ONLY_SECRET_MARKER"});
    });
    edit_payload(
        t.path(),
        "PX11-visibility-and-redaction-v1",
        "expected",
        |v| {
            v["assertions"][0]["expected_value"] =
                json!({"fixture":"SYNTHETIC_ONLY_SECRET_MARKER"});
        },
    );
    reject(t.path(), "sensitive marker leaked");
}
#[test]
fn declared_rejection_cannot_be_hidden_in_queue_totals() {
    let t = tree();
    edit_payload(t.path(), "PX12-gauge-and-queue-v1", "expected", |v| {
        v["assertions"][0]["expected_value"]["rejected"] = json!(0);
    });
    reject(t.path(), "queue count oracle inconsistent");
}
#[test]
fn incomplete_streams_cannot_claim_exact_device_idle() {
    let t = tree();
    edit_payload(
        t.path(),
        "PX09-incomplete-stream-coverage-v1",
        "expected",
        |v| {
            v["assertions"][0]["kind"] = json!("device_idle");
            v["assertions"][0]["state"] = json!("known");
            v["assertions"][0]["unit"] = json!("microseconds");
            v["assertions"][0]["expected_value"] = json!(10);
        },
    );
    reject(t.path(), "device idle needs complete coverage");
}
#[test]
fn incomplete_group_cannot_claim_exact_skew() {
    let t = tree();
    edit_payload(t.path(), "PX08-missing-rank-v1", "expected", |v| {
        v["assertions"][0]["kind"] = json!("arrival_skew");
        v["assertions"][0]["state"] = json!("known");
        v["assertions"][0]["unit"] = json!("microseconds");
        v["assertions"][0]["expected_value"] = json!(30);
    });
    reject(t.path(), "exact skew needs complete unique membership");
}
#[test]
fn temporal_record_missing_clock_is_not_assumed_local() {
    let t = tree();
    edit_payload(t.path(), "PX09-overlapping-streams-v1", "input", |v| {
        v["records"][0]
            .as_object_mut()
            .unwrap()
            .remove("clock_domain");
    });
    reject(t.path(), "temporal record requires a declared clock domain");
}
#[test]
fn cross_clock_streams_cannot_claim_exact_device_coverage() {
    let t = tree();
    edit_payload(t.path(), "PX09-overlapping-streams-v1", "input", |v| {
        v["clocks"].as_array_mut().unwrap().push(json!({"domain":"remote","unit":"microseconds","uncertainty_us":5,"offset_bounds_us":null}));
        v["records"][2]["clock_domain"] = json!("remote");
    });
    reject(
        t.path(),
        "exact device coverage requires compatible supplied clocks",
    );
}
