//! Rust-only evidence authoring after a successful recorded verification run.
use aether_poc::{fixture, observation};
use aether_validation::sha256;
use serde_json::{Value, json};
use std::{collections::BTreeSet, fs, io::Write, path::Path};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::current_dir()?;
    let report_path = "evidence/prototype/milestone-4/verify-milestone-1.json";
    let bytes = fs::read(root.join(report_path))?;
    let mut report: Value = serde_json::from_slice(&bytes)?;
    let bundle = fixture::load_bundle(&root, Path::new("fixtures/prototype/manifest.json"))?;
    if report["functional_checks_passed"] != true
        || report["milestone"] != 4
        || report["synthetic"] != true
        || report["manifest_sha256"] != bundle.manifest_sha256
        || report["runtime_acceptance_readiness"] != false
        || report["px_runtime_acceptance_status"] != "blocked_unagreed_budget"
    {
        return Err("refusing incomplete or overstated evidence".into());
    }
    let groups = report["groups"].as_array().ok_or("missing groups")?;
    if groups.len() != bundle.manifest.fixtures.len() {
        return Err("group coverage differs".into());
    }
    let mut ids = BTreeSet::new();
    for group in &bundle.manifest.fixtures {
        let matches: Vec<_> = groups
            .iter()
            .filter(|g| g["experiment_id"] == group.experiment_id)
            .collect();
        if matches.len() != 1 {
            return Err("missing/duplicate group".into());
        }
        let entries = matches[0]["variants"]
            .as_array()
            .ok_or("missing variants")?;
        if entries.len() != group.variants.len() {
            return Err("variant coverage differs".into());
        }
        for variant in &group.variants {
            let entries: Vec<_> = entries
                .iter()
                .filter(|v| v["variant_id"] == variant.id)
                .collect();
            if entries.len() != 1 {
                return Err("missing/duplicate variant".into());
            }
            let entry = entries[0];
            let expected = fixture::load_expected(&bundle, &variant.id)?;
            if entry["oracle_sha256"] != expected.sha256 {
                return Err("oracle identity differs".into());
            }
            let checks = entry["checks"].as_array().ok_or("missing checks")?;
            if checks.len() != expected.assertions.len() {
                return Err("assertion coverage differs".into());
            }
            for (index, assertion) in expected.assertions.iter().enumerate() {
                let id = format!(
                    "{}:{index}:{}:{}",
                    variant.id, assertion.kind, assertion.target
                );
                if checks[index]["assertion_id"] != id
                    || !ids.insert(id)
                    || ![
                        "matched",
                        "expected_rejection",
                        "proof_verified",
                        "deferred",
                    ]
                    .contains(
                        &checks[index]["functional_check_status"]
                            .as_str()
                            .unwrap_or(""),
                    )
                {
                    return Err("mismatched/unaccounted assertion".into());
                }
            }
        }
    }
    let evidence = json!({"path":report_path,"sha256":sha256(&bytes),"claim":"recorded functional checks; runtime acceptance blocked; no deployed validation"});
    for group in report["groups"].as_array_mut().ok_or("missing groups")? {
        for variant in group["variants"].as_array_mut().ok_or("missing variants")? {
            for check in variant["checks"].as_array_mut().ok_or("missing checks")? {
                check["evidence"] = evidence.clone();
            }
        }
    }
    let mut inputs = Vec::new();
    for path in [
        "fixtures/prototype/manifest.json",
        "evidence/manifest-v1.json",
        "docs/prototype-experiments.json",
        "docs/prototype-status.json",
        "docs/prototype-fixture-status.json",
    ] {
        inputs.push(json!({"path":path,"sha256":sha256(&fs::read(root.join(path))?)}));
    }
    for group in &bundle.manifest.fixtures {
        let path = &group.budget_path;
        inputs.push(json!({"path":path,"sha256":sha256(&fs::read(root.join(path))?)}));
    }
    report["artifact_input_identities"] = json!(inputs);
    report["evidence"] = evidence;
    report["tracking_note"] = json!(
        "This ledger records current function execution only. Accepted M1/M2/M3 planning and budget records are preserved; no whole-PX runtime pass is granted."
    );
    observation::sanitize(&mut report);
    let mut out = serde_json::to_vec_pretty(&report)?;
    out.push(b'\n');
    let dest = root.join("docs/prototype-metric-execution-status.json");
    if dest.exists() {
        if fs::read(dest)? != out {
            return Err("refusing to overwrite differing functional ledger".into());
        }
    } else {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dest)?;
        file.write_all(&out)?;
    }
    println!(
        "functional_assertions_indexed={} runtime_acceptance_readiness=false",
        ids.len()
    );
    Ok(())
}
