use crate::*;
use sha1::{Digest, Sha1};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct EvidenceSummary {
    pub sources: usize,
    pub artifacts: usize,
    pub historical_u: usize,
}

pub fn check_evidence(root: &Path) -> ValidationResult<EvidenceSummary> {
    let file = "evidence/manifest-v1.json";
    let bytes = read(root, file)?;
    let checksum = text(root, "evidence/manifest-v1.sha256")?;
    require(
        checksum.split_whitespace().next() == Some(&sha256(&bytes)),
        file,
        "manifest SHA-256 mismatch",
    )?;
    let m: Value = serde_json::from_slice(&bytes).map_err(|e| error(file, e))?;
    require(
        m["bundle_id"] == "aether-source-evidence-v1",
        file,
        "unexpected bundle ID",
    )?;
    let artifacts = array(&m, "artifacts")?;
    let mut paths = BTreeSet::new();
    for a in artifacts {
        let path = string(a, "path")?;
        require(paths.insert(path), path, "duplicate artifact path")?;
        let bytes = read(root, path)?;
        require(
            a["bytes"].as_u64() == Some(bytes.len() as u64) && a["sha256"] == sha256(&bytes),
            path,
            "artifact checksum mismatch",
        )?;
    }
    let sources = array(&m, "source_inputs")?;
    let source_ids = ids(sources, "id")?;
    require(
        source_ids.len() == 10 && sources.len() == 10,
        file,
        "expected ten unique source inputs",
    )?;
    for e in sources {
        let commit = string(e, "commit")?;
        require(
            commit.len() == 40
                && commit
                    .bytes()
                    .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()),
            file,
            "unpinned commit",
        )?;
        let expected_url = format!(
            "https://github.com/{}/blob/{}/{}",
            string(e, "repository")?,
            commit,
            string(e, "upstream_path")?
        );
        require(
            e["url"] == expected_url,
            file,
            "source URL must pin repository, commit and path",
        )?;
        let path = string(e, "snapshot")?;
        require(
            paths.contains(path),
            path,
            "source missing from artifact checksums",
        )?;
        let bytes = read(root, path)?;
        let mut blob = Sha1::new();
        blob.update(format!("blob {}\0", bytes.len()));
        blob.update(&bytes);
        require(
            e["blob_sha1"] == format!("{:x}", blob.finalize())
                && e["sha256"] == sha256(&bytes)
                && e["bytes"].as_u64() == Some(bytes.len() as u64),
            path,
            "Git blob/content mismatch",
        )?;
    }
    for key in ["source_metadata", "environment", "reconciliation"] {
        require(
            paths.contains(string(&m, key)?),
            file,
            "referenced metadata omitted from checksums",
        )?;
    }
    let index = json(root, string(&m, "source_metadata")?)?;
    require(
        index["entries"] == m["source_inputs"],
        file,
        "source index and manifest disagree",
    )?;
    let local = array(&m, "local_inputs")?;
    ids(local, "id")?;
    for entry in local {
        require(
            paths.contains(string(entry, "snapshot")?),
            file,
            "local input omitted from checksums",
        )?;
    }
    let b = load_baselines(root)?;
    let cat = array(&b.catalog, "metrics")?;
    let cov = array(&b.coverage, "metrics")?;
    let cat_ids = ids(cat, "id")?;
    let cov_ids = ids(cov, "id")?;
    require(
        cat.len() == 250 && cov.len() == 250 && cat_ids == cov_ids,
        file,
        "expected 250 original metric IDs",
    )?;
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for row in cov {
        *counts.entry(string(row, "route")?.into()).or_default() += 1;
    }
    require(
        counts
            == BTreeMap::from([
                ("E".into(), 88),
                ("R".into(), 35),
                ("H".into(), 100),
                ("I".into(), 26),
                ("U".into(), 1),
            ]),
        file,
        "unexpected route totals",
    )?;
    let r = json(root, string(&m, "reconciliation")?)?;
    let rows = array(&r, "entries")?;
    let row_ids = ids(rows, "id")?;
    let changes = b.coverage["source_mapping_review"]["changes"]
        .as_array()
        .ok_or_else(|| error(file, "missing historical changes"))?;
    let historical: BTreeSet<String> = changes
        .iter()
        .filter(|c| c["before_route"] == "U")
        .map(|c| string(c, "id").map(str::to_owned))
        .collect::<ValidationResult<_>>()?;
    require(
        rows.len() == 26 && row_ids == historical,
        file,
        "26 historical U mappings must be reconciled",
    )?;
    require(
        rows.iter().filter(|c| c["current_route"] == "E").count() == 25
            && rows.iter().filter(|c| c["current_route"] == "U").count() == 1,
        file,
        "historical U outcomes disagree",
    )?;
    for row in rows {
        let id = string(row, "id")?;
        let c = cov
            .iter()
            .find(|c| c["id"] == id)
            .ok_or_else(|| error(id, "unknown metric"))?;
        require(c["route"] == row["current_route"], id, "route mismatch")?;
        require(
            row["deployed_schema_verification"] == "not_run"
                && row["capability_verification"] == "not_run",
            id,
            "must not claim deployed verification",
        )?;
        let evidence = strings(&row["pinned_evidence_ids"])?;
        require(
            !evidence.is_empty() && evidence.is_subset(&source_ids),
            id,
            "invalid pinned evidence IDs",
        )?;
        let checks = array(row, "native_name_checks")?;
        require(!checks.is_empty(), id, "missing native-name checks")?;
        for check in checks {
            let hits = strings(&check["present_in_inputs"])?;
            require(
                !hits.is_empty() && hits.is_subset(&evidence),
                id,
                "native-name evidence missing or unrelated",
            )?;
            for eid in hits {
                let e = sources
                    .iter()
                    .find(|s| s["id"] == eid)
                    .ok_or_else(|| error(id, "unknown source"))?;
                require(
                    text(root, string(e, "snapshot")?)?.contains(string(check, "native_name")?),
                    id,
                    "native name absent",
                )?;
            }
        }
    }
    let env = json(root, string(&m, "environment")?)?;
    require(
        env["kind"] == "synthetic_local_reference_environment"
            && env["readiness"]["real_deployment_available"] == false
            && env["readiness"]["ready_for_acceptance_run"] == false,
        file,
        "environment cannot claim real deployment or acceptance readiness",
    )?;
    Ok(EvidenceSummary {
        sources: sources.len(),
        artifacts: artifacts.len(),
        historical_u: rows.len(),
    })
}
