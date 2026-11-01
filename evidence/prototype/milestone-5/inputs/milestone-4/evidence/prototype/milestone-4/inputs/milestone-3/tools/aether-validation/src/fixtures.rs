//! Construction checks only: no engine execution or live I/O.
use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureManifest {
    pub schema_version: u32,
    pub bundle_id: String,
    pub synthetic: bool,
    pub source_manifest_sha256: String,
    pub registry_sha256: String,
    pub status_sha256: String,
    pub environment_id: String,
    pub fixtures: Vec<FixtureGroup>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureGroup {
    pub fixture_id: String,
    pub experiment_id: String,
    pub requirement_ids: Vec<String>,
    pub association_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub pinned_source_ids: Vec<String>,
    pub local_baseline_ids: Vec<String>,
    pub evidence_kind: String,
    pub invented_fields: String,
    pub limits_of_claim: String,
    pub budget_path: String,
    pub budget_sha256: String,
    pub variants: Vec<FixtureVariant>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureVariant {
    pub id: String,
    pub purpose: String,
    pub intent: String,
    pub expected_payload_error: Option<String>,
    pub experiment_execution_status: String,
    pub artifacts: Vec<FixtureArtifact>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FixtureArtifact {
    pub path: String,
    pub role: String,
    pub media_type: String,
    pub bytes: u64,
    pub sha256: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureInput {
    pub schema_version: u32,
    pub synthetic: bool,
    pub variant_id: String,
    pub clocks: Vec<FixtureClock>,
    pub records: Vec<Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureClock {
    pub domain: String,
    pub unit: String,
    pub uncertainty_us: u64,
    pub offset_bounds_us: Option<[i64; 2]>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureExpected {
    pub schema_version: u32,
    pub synthetic: bool,
    pub variant_id: String,
    pub experiment_execution_status: String,
    pub assertions: Vec<FixtureAssertion>,
    pub proof: Option<Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureAssertion {
    pub kind: String,
    pub target: String,
    pub expected_value: Value,
    pub unit: String,
    pub state: String,
    pub record_ids: Vec<String>,
    pub reason: String,
}
#[derive(Debug)]
pub struct FixtureSummary {
    pub groups: usize,
    pub variants: usize,
    pub files: usize,
    pub invalid_payloads: usize,
}

pub(crate) fn decode<T: serde::de::DeserializeOwned>(
    root: &Path,
    path: &str,
) -> ValidationResult<T> {
    serde_json::from_slice(&read(root, path)?).map_err(|e| error(path, e))
}
pub(crate) fn fixture_root(root: &Path) -> ValidationResult<PathBuf> {
    safe_path(root, "fixtures/prototype")
}
pub(crate) fn validate_envelope(input: &FixtureInput, id: &str) -> ValidationResult<()> {
    require(
        input.schema_version == 1 && input.synthetic && input.variant_id == id,
        id,
        "input must identify synthetic variant/schema",
    )?;
    require(
        !input.clocks.is_empty() && input.clocks.len() <= 8,
        id,
        "invalid clock count",
    )?;
    let mut clocks = BTreeSet::new();
    for c in &input.clocks {
        require(
            !c.domain.is_empty() && clocks.insert(&c.domain) && c.unit == "microseconds",
            id,
            "invalid/duplicate clock or units",
        )?;
        if let Some([lo, hi]) = c.offset_bounds_us {
            require(lo <= hi, id, "invalid clock offset bounds")?;
        }
    }
    require(
        !input.records.is_empty() && input.records.len() <= 64,
        id,
        "records must be bounded and nonempty",
    )?;
    require(
        input
            .records
            .iter()
            .filter(|r| r["kind"] == "identity")
            .count()
            <= 32,
        id,
        "invented entity envelope exceeded",
    )?;
    let record_ids = ids(&input.records, "id")?;
    require(
        record_ids.len() == input.records.len(),
        id,
        "duplicate record identity",
    )?;
    for r in &input.records {
        require(
            r.get("captured").is_none_or(|v| v == false)
                && r.get("synthetic").is_none_or(|v| v == true)
                && r.get("origin").is_none_or(|v| v == "synthetic"),
            id,
            "record cannot claim real/captured provenance",
        )?;
        let kind = string(r, "kind")?;
        if matches!(kind, "interval" | "rank_event") || r.get("time_us").is_some() {
            require(
                r["clock_domain"]
                    .as_str()
                    .is_some_and(|s| clocks.contains(&s.to_owned())),
                id,
                "temporal record requires a declared clock domain",
            )?;
        }
        require(
            matches!(
                kind,
                "endpoint_snapshot"
                    | "counter"
                    | "identity"
                    | "link"
                    | "memory"
                    | "rank_event"
                    | "interval"
                    | "stage"
                    | "fault"
                    | "policy"
                    | "queue"
                    | "aggregate"
                    | "gauge"
            ),
            id,
            "unknown record kind",
        )?;
        if let Some(t) = r.get("time_us") {
            require(
                t.as_u64().is_some_and(|n| n <= 60_000_000),
                id,
                "invalid virtual timestamp",
            )?;
        }
        if let Some(c) = r.get("clock_domain") {
            require(
                c.as_str().is_some_and(|s| clocks.contains(&s.to_owned())),
                id,
                "unknown record clock",
            )?;
        }
        if kind == "stage" {
            for key in ["admitted", "persisted", "committed", "query_visible"] {
                require(
                    r[key].is_boolean(),
                    id,
                    "stage flags must be explicit booleans",
                )?;
            }
            require(
                !(r["committed"] == true && r["persisted"] != true)
                    && !(r["query_visible"] == true && r["committed"] != true),
                id,
                "stage visibility exceeds persistence/commit boundary",
            )?;
        }
        if kind == "rank_event" {
            require(
                r["arrival_us"]
                    .as_u64()
                    .zip(r["completion_us"].as_u64())
                    .is_some_and(|(a, b)| a <= b && b <= 60_000_000),
                id,
                "invalid rank timestamps",
            )?;
        }
        match kind {
            "identity" => {
                for k in ["entity", "incarnation"] {
                    require(
                        !string(r, k)?.is_empty(),
                        id,
                        "identity/incarnation missing",
                    )?;
                }
                let lo = r["valid_from_us"].as_u64();
                let hi = r["valid_to_us"].as_u64();
                require(
                    lo.zip(hi).is_some_and(|(l, h)| l < h && h <= 60_000_000),
                    id,
                    "invalid identity validity interval",
                )?;
            }
            "interval" => require(
                r["start_us"]
                    .as_u64()
                    .zip(r["end_us"].as_u64())
                    .is_some_and(|(a, b)| a < b && b <= 60_000_000),
                id,
                "invalid half-open interval",
            )?,
            "memory" => require(
                r["unit"] == "MiB"
                    && r["used"]
                        .as_u64()
                        .zip(r["total"].as_u64())
                        .is_some_and(|(u, t)| t > 0 && u <= t && r["free"].as_u64() == Some(t - u)),
                id,
                "memory units/capacity inconsistent",
            )?,
            "counter" | "gauge" => require(
                r["value"]
                    .as_f64()
                    .is_some_and(|n| n.is_finite() && n >= 0.0)
                    && !string(r, "unit")?.is_empty(),
                id,
                "invalid finite observation or units",
            )?,
            "rank_event" => {
                for k in ["group", "generation", "clock_domain"] {
                    require(!string(r, k)?.is_empty(), id, "rank context missing")?;
                }
                require(r["rank"].as_u64().is_some(), id, "rank missing")?;
            }
            "fault" => require(
                matches!(
                    r["action"].as_str(),
                    Some(
                        "write_failure"
                            | "commit_visibility_interruption"
                            | "duplicate_retry"
                            | "volatile_loss"
                    )
                ),
                id,
                "fault is not an allowlisted data action",
            )?,
            _ => {}
        }
    }
    // Entity lifetime duplicates conflict even when record IDs are different.
    let mut lifetimes = BTreeMap::<(String, String), (u64, u64)>::new();
    for r in &input.records {
        if r["kind"] == "identity" {
            let key = (
                string(r, "entity")?.into(),
                string(r, "incarnation")?.into(),
            );
            let bounds = (
                r["valid_from_us"].as_u64().unwrap_or(0),
                r["valid_to_us"].as_u64().unwrap_or(0),
            );
            require(
                lifetimes.insert(key, bounds).is_none(),
                id,
                "duplicate entity incarnation",
            )?;
        }
    }
    Ok(())
}
fn from_vec(items: &[String], id: &str) -> ValidationResult<BTreeSet<String>> {
    let set: BTreeSet<_> = items.iter().cloned().collect();
    require(set.len() == items.len(), id, "duplicate reference")?;
    Ok(set)
}
fn scan_files(root: &Path, dir: &Path, set: &mut BTreeSet<String>) -> ValidationResult<()> {
    require(
        dir.components()
            .count()
            .saturating_sub(root.components().count())
            <= 8,
        "fixture tree",
        "directory depth exceeds fixture envelope",
    )?;
    for entry in fs::read_dir(dir).map_err(|e| error("fixture tree", e))? {
        let entry = entry.map_err(|e| error("fixture tree", e))?;
        let ty = entry.file_type().map_err(|e| error("fixture tree", e))?;
        require(
            !ty.is_symlink(),
            "fixture tree",
            "symlink packaging is not permitted",
        )?;
        if ty.is_dir() {
            scan_files(root, &entry.path(), set)?;
        } else {
            let p = entry.path();
            let r = p
                .strip_prefix(root)
                .map_err(|e| error("fixture tree", e))?
                .to_str()
                .ok_or_else(|| error("fixture tree", "non UTF-8 path"))?
                .to_string();
            require(set.insert(r), "fixture tree", "duplicate path")?;
        }
        require(
            set.len() <= 256,
            "fixture tree",
            "file count exceeds fixture envelope",
        )?;
    }
    Ok(())
}
pub fn check_fixtures(root: &Path) -> ValidationResult<FixtureSummary> {
    check_traceability(root)?;
    let base = fixture_root(root)?;
    let file = "manifest.json";
    require(
        read(&base, file)?.len() <= 1_048_576,
        file,
        "manifest too large",
    )?;
    let m: FixtureManifest = decode(&base, file)?;
    require(
        m.schema_version == 1
            && m.synthetic
            && m.bundle_id == "aether-synthetic-fixtures-v1"
            && m.environment_id == "aether-validation-synthetic-local-v1",
        file,
        "fixture bundle must be synthetic and versioned",
    )?;
    for (path, hash) in [
        ("evidence/manifest-v1.json", &m.source_manifest_sha256),
        ("docs/prototype-experiments.json", &m.registry_sha256),
        ("docs/prototype-status.json", &m.status_sha256),
    ] {
        require(
            sha256(&read(root, path)?) == *hash,
            file,
            "fixture input hash mismatch",
        )?;
    }
    let registry = json(root, "docs/prototype-experiments.json")?;
    let experiments = array(&registry, "experiments")?;
    require(m.fixtures.len() == 12, file, "missing experiment groups")?;
    let mut groups = BTreeSet::new();
    let mut variants = BTreeSet::new();
    let mut artifacts = BTreeMap::<String, FixtureArtifact>::new();
    let mut negatives = 0;
    let sources = json(root, "evidence/manifest-v1.json")?;
    for group in &m.fixtures {
        let id = &group.experiment_id;
        require(groups.insert(id.clone()), id, "duplicate experiment group")?;
        let experiment = experiments
            .iter()
            .find(|e| e["id"] == *id)
            .ok_or_else(|| error(id, "unknown experiment group"))?;
        require(
            group.fixture_id == string(experiment, "planned_fixture_id")?,
            id,
            "parent fixture ID mismatch",
        )?;
        for (values, key) in [
            (&group.requirement_ids, "requirement_ids"),
            (&group.association_ids, "association_ids"),
            (&group.profile_ids, "profile_ids"),
            (&group.pinned_source_ids, "pinned_source_ids"),
            (&group.local_baseline_ids, "local_baseline_ids"),
        ] {
            require(
                from_vec(values, id)? == strings(&experiment[key])?,
                id,
                &format!("dropped/changed registry links: {key}"),
            )?;
        }
        require(
            group.evidence_kind == string(experiment, "evidence_kind")?
                && !group.invented_fields.trim().is_empty()
                && group.limits_of_claim == string(experiment, "limits_of_claim")?,
            id,
            "invalid synthetic provenance or claim limits",
        )?;
        require(
            group.budget_path == string(experiment, "budget_record")?
                && group.budget_sha256 == sha256(&read(root, &group.budget_path)?),
            id,
            "budget path/hash mismatch",
        )?;
        require(
            group.variants.len() >= 2 && group.variants.len() <= 16,
            id,
            "missing or unbounded variants",
        )?;
        require(
            group.variants.iter().any(|v| v.intent == "positive")
                && group.variants.iter().any(|v| v.intent == "negative"),
            id,
            "each group requires positive and negative variants",
        )?;
        for variant in &group.variants {
            let vid = &variant.id;
            require(
                vid.starts_with(&format!("{id}-"))
                    && vid.ends_with("-v1")
                    && variants.insert(vid.clone()),
                vid,
                "invalid/duplicate variant ID",
            )?;
            require(
                !variant.purpose.is_empty()
                    && matches!(variant.intent.as_str(), "positive" | "negative")
                    && variant.experiment_execution_status == "not_run",
                vid,
                "invalid purpose or false runtime execution",
            )?;
            require(
                (2..=3).contains(&variant.artifacts.len()),
                vid,
                "input/expected/optional endpoint artifacts required",
            )?;
            let mut roles = BTreeMap::new();
            for a in &variant.artifacts {
                require(
                    matches!(a.role.as_str(), "input" | "expected" | "endpoint")
                        && roles.insert(a.role.clone(), a.path.clone()).is_none(),
                    vid,
                    "duplicate or unsupported artifact role",
                )?;
                require(
                    a.media_type
                        == if a.role == "endpoint" {
                            "text/plain"
                        } else {
                            "application/json"
                        },
                    vid,
                    "wrong artifact media type",
                )?;
                require(
                    a.bytes <= 1_048_576,
                    vid,
                    "artifact exceeds proposed byte limit",
                )?;
                let p = safe_path(&base, &a.path)?;
                require(
                    fs::metadata(&p).map_err(|e| error(&a.path, e))?.len() <= 1_048_576,
                    vid,
                    "actual payload exceeds byte limit",
                )?;
                let bytes = read(&base, &a.path)?;
                require(
                    bytes.len() as u64 == a.bytes && sha256(&bytes) == a.sha256,
                    &a.path,
                    "fixture artifact hash/length mismatch",
                )?;
                if let Some(old) = artifacts.insert(a.path.clone(), a.clone()) {
                    require(old == *a, vid, "shared artifact declarations conflict")?;
                }
            }
            let input_path = roles
                .get("input")
                .ok_or_else(|| error(vid, "missing input artifact"))?;
            let expected_path = roles
                .get("expected")
                .ok_or_else(|| error(vid, "missing oracle artifact"))?;
            let input: FixtureInput = decode(&base, input_path)?;
            validate_envelope(&input, vid)?;
            let oracle: FixtureExpected = decode(&base, expected_path)?;
            validate_oracle(&input, &oracle, vid)?;
            // Even intentionally malformed text must have honest native provenance.
            for declaration in input
                .records
                .iter()
                .filter(|r| r["kind"] == "endpoint_snapshot")
            {
                let ids = strings(&declaration["source_input_ids"])?;
                require(
                    !ids.is_empty()
                        && ids.is_subset(&group.pinned_source_ids.iter().cloned().collect()),
                    vid,
                    "unknown endpoint source ID",
                )?;
                let family = string(declaration, "family")?;
                let mut documented = false;
                for sid in ids {
                    let source = array(&sources, "source_inputs")?
                        .iter()
                        .find(|s| s["id"] == sid)
                        .ok_or_else(|| error(vid, "unknown pinned source ID"))?;
                    documented |= text(root, string(source, "snapshot")?)?.contains(family);
                }
                require(
                    documented,
                    vid,
                    "declared family absent from pinned definitions",
                )?;
            }
            if let Some(endpoint) = roles.get("endpoint") {
                let payload = text(&base, endpoint)?;
                let parsed = crate::fixture_prom::parse_prom(&payload);
                if let Some(code) = &variant.expected_payload_error {
                    require(
                        variant.intent == "negative"
                            && matches!(code.as_str(), "TYPE_INVALID" | "HISTOGRAM_INCONSISTENT"),
                        vid,
                        "unknown negative payload check",
                    )?;
                    match parsed {
                        Err(actual) => require(
                            actual == *code,
                            vid,
                            "negative payload rejected for wrong reason",
                        )?,
                        Ok(_) => return Err(error(vid, "negative payload was not rejected")),
                    };
                    require(
                        oracle
                            .assertions
                            .iter()
                            .any(|a| a.kind == "rejection" && a.expected_value == *code),
                        vid,
                        "negative payload oracle missing rejection code",
                    )?;
                    negatives += 1;
                } else {
                    let families = parsed.map_err(|e| error(endpoint, e))?;
                    require(families.len() <= 64, vid, "series/family envelope exceeded")?;
                    require(
                        families.values().map(|f| f.samples.len()).sum::<usize>() <= 64,
                        vid,
                        "series envelope exceeded",
                    )?;
                    for (name, family) in &families {
                        let record = input
                            .records
                            .iter()
                            .find(|r| r["family"] == *name)
                            .ok_or_else(|| error(vid, "endpoint family missing declaration"))?;
                        let declared = if record["kind"] == "counter" {
                            "counter"
                        } else {
                            string(record, "metric_type")?
                        };
                        require(
                            family.typ == declared,
                            vid,
                            "endpoint type differs from declaration",
                        )?;
                        for a in oracle
                            .assertions
                            .iter()
                            .filter(|a| a.kind == "native" && a.target == *name)
                        {
                            let values = array(&a.expected_value, "sample_values")?;
                            require(
                                values.len() == family.samples.len()
                                    && values
                                        .iter()
                                        .zip(&family.samples)
                                        .all(|(v, s)| v.as_f64() == Some(s.value)),
                                vid,
                                "native oracle sample values contradict exposition",
                            )?;
                        }
                        let ids = record
                            .get("source_input_ids")
                            .map(strings)
                            .transpose()?
                            .unwrap_or(group.pinned_source_ids.iter().cloned().collect());
                        require(
                            !ids.is_empty()
                                && ids
                                    .is_subset(&group.pinned_source_ids.iter().cloned().collect()),
                            vid,
                            "unknown endpoint source ID",
                        )?;
                        let mut found = false;
                        for sid in ids {
                            let src = array(&sources, "source_inputs")?
                                .iter()
                                .find(|e| e["id"] == sid)
                                .ok_or_else(|| error(vid, "unknown pinned source ID"))?;
                            if text(root, string(src, "snapshot")?)?.contains(name) {
                                found = true;
                            }
                        }
                        require(
                            found,
                            vid,
                            "native family absent from pinned source definitions",
                        )?;
                        if let Some(labels) = record.get("native_labels") {
                            let allowed = strings(labels)?;
                            let invented = record
                                .get("invented_labels")
                                .map(strings)
                                .transpose()?
                                .unwrap_or_default();
                            for s in &family.samples {
                                for (k, v) in &s.labels {
                                    require(
                                        k == "le" || allowed.contains(k) || invented.contains(k),
                                        vid,
                                        "undeclared native/invented label",
                                    )?;
                                    require(v.len() <= 256, vid, "label exceeds byte limit")?;
                                }
                            }
                        }
                    }
                }
            } else {
                require(
                    variant.expected_payload_error.is_none(),
                    vid,
                    "negative payload declaration without endpoint",
                )?;
            }
        }
    }
    require(
        groups == ids(experiments, "id")?,
        file,
        "missing experiment groups",
    )?;
    let mut actual = BTreeSet::new();
    scan_files(&base, &base, &mut actual)?;
    actual.remove("manifest.json");
    actual.remove("README.md");
    require(
        actual == artifacts.keys().cloned().collect(),
        file,
        "unindexed payload files or missing declarations",
    )?;
    validate_ledger(root, &m, &artifacts)?;
    Ok(FixtureSummary {
        groups: groups.len(),
        variants: variants.len(),
        files: artifacts.len(),
        invalid_payloads: negatives,
    })
}
fn record<'a>(input: &'a FixtureInput, id: &str) -> ValidationResult<&'a Value> {
    input
        .records
        .iter()
        .find(|r| r["id"] == id)
        .ok_or_else(|| error(id, "oracle references missing input record"))
}
fn num(v: &Value, key: &str) -> ValidationResult<f64> {
    v[key]
        .as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| error(key, "finite number required"))
}
fn expected_number(a: &FixtureAssertion, actual: f64) -> ValidationResult<()> {
    require(
        a.expected_value.as_f64() == Some(actual),
        &a.target,
        "expected numeric answer contradicts independent input arithmetic",
    )
}
fn population_variance(values: &Value) -> ValidationResult<f64> {
    let a = values
        .as_array()
        .ok_or_else(|| error("proof", "number array required"))?;
    require(!a.is_empty(), "proof", "empty population")?;
    let xs: Vec<f64> = a
        .iter()
        .map(|v| {
            v.as_f64()
                .filter(|n| n.is_finite())
                .ok_or_else(|| error("proof", "nonfinite observation"))
        })
        .collect::<ValidationResult<_>>()?;
    let mean = xs.iter().sum::<f64>() / xs.len() as f64;
    Ok(xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / xs.len() as f64)
}
fn validate_oracle(
    input: &FixtureInput,
    oracle: &FixtureExpected,
    id: &str,
) -> ValidationResult<()> {
    require(
        oracle.schema_version == 1
            && oracle.synthetic
            && oracle.variant_id == id
            && oracle.experiment_execution_status == "not_run",
        id,
        "oracle must be synthetic and unexecuted",
    )?;
    require(
        !oracle.assertions.is_empty() && oracle.assertions.len() <= 32,
        id,
        "oracle assertions missing or unbounded",
    )?;
    let mut targets = BTreeSet::new();
    for a in &oracle.assertions {
        require(
            !a.target.is_empty()
                && !a.unit.is_empty()
                && !a.reason.trim().is_empty()
                && targets.insert((&a.kind, &a.target)),
            id,
            "invalid/duplicate oracle assertion",
        )?;
        require(
            matches!(
                a.state.as_str(),
                "known"
                    | "unknown"
                    | "reported_only"
                    | "partial"
                    | "ambiguous"
                    | "resolved"
                    | "denied"
                    | "blocked"
                    | "rejected"
            ),
            id,
            "unknown oracle quality state",
        )?;
        for rid in &a.record_ids {
            record(input, rid)?;
        }
        match a.kind.as_str() {
            "unknown" => require(
                a.expected_value.is_null() && a.state == "unknown",
                id,
                "unknown must have null value, not a fabricated zero",
            )?,
            "native" => {
                let r = record(
                    input,
                    a.record_ids
                        .first()
                        .ok_or_else(|| error(id, "native oracle needs declaration"))?,
                )?;
                require(
                    a.expected_value["type"] == r["metric_type"]
                        && a.expected_value["scope"] == r["scope"]
                        && a.unit == r["unit"],
                    id,
                    "native oracle changes type/scope/units",
                )?;
            }
            "rejection" => require(
                matches!(
                    a.expected_value.as_str(),
                    Some("TYPE_INVALID" | "HISTOGRAM_INCONSISTENT")
                ) && a.state == "rejected",
                id,
                "unrecognized payload rejection oracle",
            )?,
            "rate" => {
                require(
                    a.record_ids.len() == 2 && a.unit == "requests/second",
                    id,
                    "rate needs two request-count snapshots and matching units",
                )?;
                let x = record(input, &a.record_ids[0])?;
                let y = record(input, &a.record_ids[1])?;
                require(
                    x["kind"] == "counter"
                        && y["kind"] == "counter"
                        && x["unit"] == "requests"
                        && y["unit"] == "requests"
                        && x["family"] == y["family"]
                        && x["incarnation"] == y["incarnation"]
                        && x["clock_domain"] == y["clock_domain"],
                    id,
                    "incompatible rate population or incarnation",
                )?;
                let dt = num(y, "time_us")? - num(x, "time_us")?;
                let dv = num(y, "value")? - num(x, "value")?;
                require(
                    dt > 0.0 && dv >= 0.0,
                    id,
                    "reset/invalid interval cannot produce exact rate",
                )?;
                expected_number(a, dv / (dt / 1_000_000.0))?;
            }
            "occupancy" => {
                let r = record(
                    input,
                    a.record_ids
                        .first()
                        .ok_or_else(|| error(id, "occupancy record missing"))?,
                )?;
                require(
                    a.unit == "ratio" && r["kind"] == "memory",
                    id,
                    "occupancy units/type mismatch",
                )?;
                expected_number(a, num(r, "used")? / num(r, "total")?)?;
            }
            "logical_count" => {
                let mut ids = BTreeSet::new();
                for rid in &a.record_ids {
                    let r = record(input, rid)?;
                    let values = strings(&r["logical_ids"])?;
                    require(
                        r["count"].as_u64() == Some(values.len() as u64),
                        id,
                        "logical population count mismatch",
                    )?;
                    ids.extend(values);
                }
                require(a.unit == "requests", id, "logical count is request scoped")?;
                expected_number(a, ids.len() as f64)?;
            }
            "replica_count" => {
                let r = record(input, "replica-count")?;
                require(
                    a.unit == "requests" && r["scope"] == a.target,
                    id,
                    "replica count scope mismatch",
                )?;
                expected_number(a, num(r, "count")?)?;
            }
            "arrival_skew" | "completion_skew" => {
                require(a.unit == "microseconds", id, "skew units mismatch")?;
                let ranks: Vec<_> = input
                    .records
                    .iter()
                    .filter(|r| r["kind"] == "rank_event")
                    .collect();
                require(!ranks.is_empty(), id, "rank events missing")?;
                let rankset: BTreeSet<_> = ranks
                    .iter()
                    .map(|r| r["rank"].as_u64().unwrap_or(u64::MAX))
                    .collect();
                let group = record(input, "group")?;
                let expected: BTreeSet<_> = array(group, "expected_ranks")?
                    .iter()
                    .filter_map(Value::as_u64)
                    .collect();
                require(
                    rankset == expected
                        && ranks.len() == rankset.len()
                        && group["complete"] == true,
                    id,
                    "exact skew needs complete unique membership",
                )?;
                require(
                    ranks.iter().all(|r| {
                        r["group"] == ranks[0]["group"]
                            && r["generation"] == ranks[0]["generation"]
                            && r["clock_domain"] == ranks[0]["clock_domain"]
                    }),
                    id,
                    "exact skew has incompatible group/clock",
                )?;
                let domain = string(ranks[0], "clock_domain")?;
                require(
                    input.clocks.iter().any(|c| {
                        c.domain == domain
                            && c.uncertainty_us == 0
                            && c.offset_bounds_us == Some([0, 0])
                    }),
                    id,
                    "exact skew requires exact supplied fixture clock",
                )?;
                let key = if a.kind == "arrival_skew" {
                    "arrival_us"
                } else {
                    "completion_us"
                };
                let values: Vec<_> = ranks
                    .iter()
                    .map(|r| num(r, key))
                    .collect::<ValidationResult<_>>()?;
                expected_number(
                    a,
                    values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                        - values.iter().copied().fold(f64::INFINITY, f64::min),
                )?;
            }
            "duration" => {
                let r = record(input, &a.target)?;
                require(a.unit == "microseconds", id, "duration units mismatch")?;
                let d = num(r, "completion_us")? - num(r, "arrival_us")?;
                require(d >= 0.0, id, "negative duration")?;
                expected_number(a, d)?;
            }
            "stream_gap" => {
                let events: Vec<_> = input
                    .records
                    .iter()
                    .filter(|r| r["kind"] == "interval" && r["stream"] == a.target)
                    .collect();
                require(
                    common_exact_clock(input, &events),
                    id,
                    "exact stream gap requires compatible supplied clocks",
                )?;
                let mut intervals: Vec<_> = input
                    .records
                    .iter()
                    .filter(|r| r["kind"] == "interval" && r["stream"] == a.target)
                    .map(|r| Ok((num(r, "start_us")?, num(r, "end_us")?)))
                    .collect::<ValidationResult<_>>()?;
                intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
                require(
                    intervals.len() == 2 && a.unit == "microseconds",
                    id,
                    "gap fixture requires two same-stream intervals",
                )?;
                expected_number(a, (intervals[1].0 - intervals[0].1).max(0.0))?;
            }
            "device_idle" => {
                let events: Vec<_> = input
                    .records
                    .iter()
                    .filter(|r| r["kind"] == "interval")
                    .collect();
                require(
                    common_exact_clock(input, &events),
                    id,
                    "exact device coverage requires compatible supplied clocks",
                )?;
                let cov = record(input, "coverage")?;
                require(
                    cov["complete_stream_coverage"] == true && a.unit == "microseconds",
                    id,
                    "device idle needs complete coverage and units",
                )?;
                let window = array(cov, "window_us")?;
                require(window.len() == 2, id, "invalid observation window")?;
                let lo = window[0]
                    .as_f64()
                    .ok_or_else(|| error(id, "invalid window"))?;
                let hi = window[1]
                    .as_f64()
                    .ok_or_else(|| error(id, "invalid window"))?;
                let mut intervals: Vec<_> = input
                    .records
                    .iter()
                    .filter(|r| r["kind"] == "interval")
                    .map(|r| Ok((num(r, "start_us")?.max(lo), num(r, "end_us")?.min(hi))))
                    .collect::<ValidationResult<_>>()?;
                intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
                let mut cursor = lo;
                let mut covered = 0.0;
                for (s, e) in intervals {
                    let start = s.max(cursor);
                    if e > start {
                        covered += e - start;
                        cursor = e;
                    }
                }
                require(hi > lo, id, "invalid observation window")?;
                expected_number(a, hi - lo - covered)?;
            }
            "committed_ids" | "visible_ids" => {
                let field = if a.kind == "committed_ids" {
                    "committed"
                } else {
                    "query_visible"
                };
                let expected: BTreeSet<_> = input
                    .records
                    .iter()
                    .filter(|r| r["kind"] == "stage" && r[field] == true)
                    .map(|r| string(r, "logical_id").map(str::to_owned))
                    .collect::<ValidationResult<_>>()?;
                require(
                    strings(&a.expected_value)? == expected && a.unit == "record_ids",
                    id,
                    "commit/visibility oracle contradicts supplied stages",
                )?;
            }
            "queue_counts" => {
                let r = record(input, &a.target)?;
                let offered = strings(&r["offered_ids"])?;
                let admitted = strings(&r["admitted_ids"])?;
                let rejected = strings(&r["rejected_ids"])?;
                require(
                    r["policy"] == "reject_new"
                        && admitted.is_disjoint(&rejected)
                        && admitted.union(&rejected).cloned().collect::<BTreeSet<_>>() == offered
                        && admitted.len() as u64 <= r["capacity_records"].as_u64().unwrap_or(0),
                    id,
                    "queue IDs do not account for offered records",
                )?;
                require(
                    a.expected_value["offered"].as_u64() == Some(offered.len() as u64)
                        && a.expected_value["admitted"].as_u64() == Some(admitted.len() as u64)
                        && a.expected_value["rejected"].as_u64() == Some(rejected.len() as u64)
                        && a.unit == "records",
                    id,
                    "queue count oracle inconsistent",
                )?;
            }
            "sanitized" => {
                let r = record(input, "visibility")?;
                require(
                    a.expected_value == r["retained_labels"],
                    id,
                    "sanitized labels mismatch",
                )?;
                let retained =
                    serde_json::to_string(&a.expected_value).map_err(|e| error(id, e))?;
                require(
                    !retained.contains(string(r, "synthetic_sensitive_marker")?),
                    id,
                    "sensitive marker leaked into oracle retained data",
                )?;
            }
            "proof" => {
                let p = oracle
                    .proof
                    .as_ref()
                    .ok_or_else(|| error(id, "oracle proof missing"))?;
                let va = population_variance(&p["variance"]["a"])?;
                let vb = population_variance(&p["variance"]["b"])?;
                let aa = array(&p["variance"], "a")?;
                let bb = array(&p["variance"], "b")?;
                require(
                    aa.len() == bb.len()
                        && aa.iter().filter_map(Value::as_f64).sum::<f64>()
                            == bb.iter().filter_map(Value::as_f64).sum::<f64>()
                        && aa
                            .iter()
                            .chain(bb)
                            .all(|v| v.as_f64().is_some_and(|x| (0.0..=1.0).contains(&x)))
                        && va != vb,
                    id,
                    "variance proof does not preserve aggregates or differing answers",
                )?;
                let g = &p["goodput"];
                let latencies = array(g, "latencies_seconds")?;
                let window = num(g, "window_seconds")?;
                let slo = num(g, "slo_seconds")?;
                require(window > 0.0, id, "goodput window invalid")?;
                let qualified = |key: &str| -> ValidationResult<f64> {
                    let outcomes = array(g, key)?;
                    require(
                        outcomes.len() == latencies.len() && outcomes.iter().all(Value::is_boolean),
                        id,
                        "goodput pairing invalid",
                    )?;
                    Ok(latencies
                        .iter()
                        .zip(outcomes)
                        .filter(|(l, s)| {
                            l.as_f64().is_some_and(|v| v <= slo) && s.as_bool() == Some(true)
                        })
                        .count() as f64
                        / window)
                };
                let ga = qualified("success_a")?;
                let gb = qualified("success_b")?;
                require(
                    array(g, "success_a")?
                        .iter()
                        .filter(|v| **v == true)
                        .count()
                        == array(g, "success_b")?
                            .iter()
                            .filter(|v| **v == true)
                            .count()
                        && ga != gb,
                    id,
                    "goodput worlds must have equal success counts and different joint outcomes",
                )?;
                require(
                    a.expected_value["variance_a"].as_f64() == Some(va)
                        && a.expected_value["variance_b"].as_f64() == Some(vb)
                        && a.expected_value["goodput_a"].as_f64() == Some(ga)
                        && a.expected_value["goodput_b"].as_f64() == Some(gb),
                    id,
                    "proof oracle number inconsistent",
                )?;
            }
            "state" => match a.target.as_str() {
                "publisher" => require(
                    a.expected_value == record(input, "listener")?["publisher_candidates"]
                        && a.state == "ambiguous",
                    id,
                    "ambiguous publisher oracle inconsistent",
                )?,
                "composition" => require(
                    a.expected_value == record(input, "partial")?["members"]
                        && a.state == "partial",
                    id,
                    "partial composition oracle inconsistent",
                )?,
                "historical_owner" => require(
                    a.expected_value == record(input, "old-membership")?["container"],
                    id,
                    "historical ownership oracle inconsistent",
                )?,
                "capabilities" => require(
                    a.expected_value == record(input, "capability")?["capabilities"]
                        && a.state == "reported_only",
                    id,
                    "capability oracle must remain reported-only",
                )?,
                "lost_ids" => {
                    let lost: BTreeSet<_> = input
                        .records
                        .iter()
                        .filter(|r| {
                            r["kind"] == "stage" && r["admitted"] == true && r["persisted"] == false
                        })
                        .map(|r| string(r, "logical_id").map(str::to_owned))
                        .collect::<ValidationResult<_>>()?;
                    require(
                        strings(&a.expected_value)? == lost,
                        id,
                        "lost IDs inconsistent",
                    )?;
                }
                "actions" => require(
                    a.expected_value == "rejected" && a.state == "denied",
                    id,
                    "forbidden actions cannot be permitted",
                )?,
                "runtime_readiness" => require(
                    a.expected_value == false
                        && record(input, "actions")?["required_limit"].is_null()
                        && record(input, "actions")?["runtime_readiness"] == false,
                    id,
                    "unagreed budget cannot become ready",
                )?,
                _ => return Err(error(id, "unknown state assertion target")),
            },
            _ => return Err(error(id, "unsupported oracle assertion kind")),
        }
    }
    Ok(())
}
fn validate_ledger(
    root: &Path,
    m: &FixtureManifest,
    artifacts: &BTreeMap<String, FixtureArtifact>,
) -> ValidationResult<()> {
    let path = "docs/prototype-fixture-status.json";
    let ledger = json(root, path)?;
    require(
        ledger["schema_version"] == 1
            && ledger["synthetic"] == true
            && ledger["manifest_sha256"]
                == sha256(&read(root, "fixtures/prototype/manifest.json")?)
            && ledger["registry_sha256"] == m.registry_sha256
            && ledger["status_sha256"] == m.status_sha256,
        path,
        "fixture ledger input/bundle hashes mismatch",
    )?;
    require(
        ledger["runtime_acceptance_readiness"] == false
            && ledger["deployed_schema_verification"] == "not_run"
            && ledger["capability_verification"] == "not_run",
        path,
        "ledger cannot claim runtime/deployed readiness",
    )?;
    let entries = array(&ledger, "entries")?;
    require(
        ids(entries, "experiment_id")?
            == m.fixtures.iter().map(|g| g.experiment_id.clone()).collect(),
        path,
        "ledger groups missing/duplicate",
    )?;
    for entry in entries {
        let id = string(entry, "experiment_id")?;
        let g = m
            .fixtures
            .iter()
            .find(|g| g.experiment_id == id)
            .ok_or_else(|| error(path, "unknown group"))?;
        require(
            entry["fixture_id"] == g.fixture_id
                && strings(&entry["variant_ids"])?
                    == g.variants.iter().map(|v| v.id.clone()).collect(),
            id,
            "ledger variants disagree",
        )?;
        let expected: BTreeSet<_> = g
            .variants
            .iter()
            .flat_map(|v| v.artifacts.iter().map(|a| a.path.clone()))
            .collect();
        require(
            strings(&entry["files"])? == expected
                && expected.iter().all(|p| artifacts.contains_key(p)),
            id,
            "ledger file links disagree",
        )?;
        require(
            entry["authoring_status"] == "authored"
                && entry["experiment_execution_status"] == "not_run",
            id,
            "ledger authoring/runtime state mismatch",
        )?;
        match entry["fixture_validation_status"].as_str() {
            Some("not_run") => require(
                entry["fixture_validation_evidence"].is_null(),
                id,
                "unrun construction check cannot have actual evidence",
            )?,
            Some("passed") => {
                let e = &entry["fixture_validation_evidence"];
                let p = string(e, "path")?;
                require(
                    e["sha256"] == sha256(&read(root, p)?)
                        && text(root, p)?.contains("milestone_2_fixture_validation=pass"),
                    id,
                    "construction pass evidence missing or mismatched",
                )?;
            }
            _ => return Err(error(id, "invalid construction validation state")),
        }
    }
    Ok(())
}

fn common_exact_clock(input: &FixtureInput, records: &[&Value]) -> bool {
    let Some(first) = records.first() else {
        return false;
    };
    let Some(domain) = first["clock_domain"].as_str() else {
        return false;
    };
    records.iter().all(|r| r["clock_domain"] == domain)
        && input
            .clocks
            .iter()
            .any(|c| c.domain == domain && c.uncertainty_us == 0)
}
