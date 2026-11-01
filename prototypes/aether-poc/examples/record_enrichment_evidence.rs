//! Index recorded M5 function evidence; preserve the historical M4 ledger.
use aether_poc::{enrichment_fixture, fixture, observation};
use aether_validation::{parse_association_rows, sha256};
use serde_json::{Value, json};
use std::{collections::BTreeSet, fs, io::Write, path::Path};
fn protected(path: &Path, value: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    if path.exists() {
        if fs::read(path)? != bytes {
            return Err("refusing conflicting evidence output".into());
        }
    } else {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        f.write_all(&bytes)?;
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::current_dir()?;
    let report_path = "evidence/prototype/milestone-5/verify-milestone-1.json";
    let bytes = fs::read(root.join(report_path))?;
    let mut report: Value = serde_json::from_slice(&bytes)?;
    let bundle = fixture::load_bundle(&root, Path::new("fixtures/prototype/manifest.json"))?;
    let extension = enrichment_fixture::load_enrichment_bundle(&root)?;
    if report["milestone"] != 5
        || report["functional_checks_passed"] != true
        || report["runtime_acceptance_readiness"] != false
        || report["manifest_sha256"] != bundle.manifest_sha256
        || report["extension_manifest_sha256"] != extension.manifest_sha256
        || report["original_assertions_indexed"] != 49
        || report["current_original_matches"] != 17
        || report["new_original_matches"] != 7
    {
        return Err("incomplete or overstated report".into());
    }
    let mut ids = BTreeSet::new();
    for group in &bundle.manifest.fixtures {
        let gs = report["groups"].as_array().ok_or("missing groups")?;
        let matches: Vec<_> = gs
            .iter()
            .filter(|g| g["experiment_id"] == group.experiment_id)
            .collect();
        if matches.len() != 1 {
            return Err("missing/duplicate group".into());
        }
        let variants = matches[0]["variants"]
            .as_array()
            .ok_or("missing variants")?;
        if variants.len() != group.variants.len() {
            return Err("variant count differs".into());
        }
        for v in &group.variants {
            let matches: Vec<_> = variants
                .iter()
                .filter(|p| p["variant_id"] == v.id)
                .collect();
            if matches.len() != 1 {
                return Err("missing/duplicate variant".into());
            }
            let actual = matches[0];
            let e = fixture::load_expected(&bundle, &v.id)?;
            if actual["oracle_sha256"] != e.sha256 {
                return Err("oracle hash mismatch".into());
            }
            let checks = actual["checks"].as_array().ok_or("missing checks")?;
            if checks.len() != e.assertions.len() {
                return Err("original assertion count differs".into());
            }
            for (index, a) in e.assertions.iter().enumerate() {
                let id = format!("{}:{index}:{}:{}", v.id, a.kind, a.target);
                if checks[index]["assertion_id"] != id || !ids.insert(id) {
                    return Err("assertion identity differs".into());
                }
            }
        }
    }
    let boundary = report["boundary_cases"]
        .as_array()
        .ok_or("missing boundary cases")?;
    if boundary.len() != 7 {
        return Err("boundary coverage differs".into());
    }
    let mut extensions = BTreeSet::new();
    for case in boundary {
        let id = case["case_id"].as_str().ok_or("missing case id")?;
        if !extension.inputs.contains_key(id)
            || !extensions.insert(id.to_string())
            || case["passed"] != true
            || !case["checks"]
                .as_object()
                .ok_or("missing boundary checks")?
                .values()
                .all(|v| *v == true)
        {
            return Err("boundary mismatch".into());
        }
    }
    let evidence = json!({"path":report_path,"sha256":sha256(&bytes),"claim":"synthetic association function evidence; runtime budgets blocked and real qualification not run"});
    report["evidence"] = evidence.clone();
    report["tracking_note"] = json!(
        "Unique original assertion IDs remain 49. Extension checks are separate. Prior-only checks link preserved M4 evidence and were not executed by this M5 run."
    );
    observation::sanitize(&mut report);
    protected(
        &root.join("docs/prototype-enrichment-execution-status.json"),
        &report,
    )?;
    let table = fs::read_to_string(root.join("metadata-enrichment-matrix.md"))?;
    let rows = parse_association_rows(&table).map_err(|_| "association table invalid")?;
    let mut entries = Vec::new();
    for row in rows {
        let id = row["id"].as_str().ok_or("missing association id")?;
        let facet = match id {
            "M02" | "M30" => Some("publisher candidates and unresolved collection-time ownership"),
            "M11" | "M12" | "M31" => {
                Some("remote subject retained without guessed publisher-local host")
            }
            "M22" | "M24" => {
                Some("explicit invented host/boot/namespace distinctions; no discovery")
            }
            "M23" | "M25" | "M26" => Some(
                "symbolic process incarnation/cgroup/container validity; parent/runtime qualification absent",
            ),
            "M09" | "M37" | "M40" => Some(
                "declared resource sets, partial composition and invariant measurement cardinality",
            ),
            "M32" => Some(
                "symbolic models retained; physical/partition/execution-use identity unverified",
            ),
            "M43" | "M44" => Some(
                "scoped symbolic NIC identity; model facts not joined without explicit evidence",
            ),
            "M47" | "M49" | "M50" | "M52" => Some(
                "capability/configuration distinct from unknown payload transport/operation attribution",
            ),
            "M55" | "M56" => {
                Some("unattached reported driver and unknown loaded communication/runtime versions")
            }
            "M58" => Some("denied/disabled/unknown visibility and safe simulated policy outcomes"),
            _ => None,
        };
        entries.push(json!({"association_id":id,"original_definition":row,"facet":facet,"facet_status":if facet.is_some(){"synthetic_facet_exercised"}else{"not_exercised_in_m5"},"evidence":if facet.is_some(){vec![evidence.clone()]}else{vec![]},"temporal_limits":"Only explicit compatible validity is resolved; untimed facts stay unspecified.","physical_identity_status":"unverified_synthetic","remaining":"Full association/source acquisition and production qualification remain unverified; fixture facet evidence is not complete support."}));
    }
    let coverage = json!({"schema_version":1,"synthetic":true,"association_definition_path":"metadata-enrichment-matrix.md","association_definition_sha256":sha256(table.as_bytes()),"association_count":entries.len(),"entries":entries,"runtime_acceptance_readiness":false,"real_verification":"not_run"});
    protected(
        &root.join("docs/prototype-enrichment-coverage.json"),
        &coverage,
    )?;
    println!(
        "original_assertions_indexed={} boundary_cases={} associations_retained=58 runtime_acceptance_readiness=false",
        ids.len(),
        extensions.len()
    );
    Ok(())
}
