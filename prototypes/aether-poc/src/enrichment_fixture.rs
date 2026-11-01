use crate::{
    PrototypeError, Result,
    fixture::{self, Artifact, MAX_BYTES},
    identity::*,
};
use aether_validation::sha256;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryInput {
    pub schema_version: u32,
    pub synthetic: bool,
    pub case_id: String,
    pub records: Vec<MetadataRecord>,
    pub queries: Vec<AssociationQuery>,
    pub measurements: Vec<EnrichmentInputMeasurement>,
    pub measurement_queries: BTreeMap<String, String>,
    pub updates: Vec<MetadataRecord>,
    pub after_queries: Vec<AssociationQuery>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedAssociation {
    pub state: crate::observation::QualityState,
    pub candidates: Vec<String>,
    pub temporal_status: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryExpected {
    pub schema_version: u32,
    pub synthetic: bool,
    pub case_id: String,
    pub queries: BTreeMap<String, ExpectedAssociation>,
    pub measurement_values: BTreeMap<String, Value>,
    pub update_error: Option<String>,
    pub history_unchanged: bool,
    pub permutation_invariant: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaseIndex {
    pub case_id: String,
    pub input: Artifact,
    pub expected: Artifact,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtensionManifest {
    pub schema_version: u32,
    pub synthetic: bool,
    pub bundle_id: String,
    pub original_manifest_sha256: String,
    pub associations_sha256: String,
    pub registry_sha256: String,
    pub cases: Vec<CaseIndex>,
    pub limits_of_claim: String,
}
pub struct EnrichmentFixtureBundle {
    pub root: PathBuf,
    pub manifest: ExtensionManifest,
    pub manifest_sha256: String,
    pub inputs: BTreeMap<String, BoundaryInput>,
}
fn error(s: &str) -> PrototypeError {
    PrototypeError::new("enrichment_fixture_invalid", s)
}
fn read_artifact(root: &Path, a: &Artifact, role: &str) -> Result<Vec<u8>> {
    if a.role != role || a.media_type != "application/json" || a.bytes > MAX_BYTES {
        return Err(error("artifact role/media/limit"));
    }
    let path = fixture::confined(root, Path::new(&a.path))?;
    let bytes = fixture::bounded_read(&path, a.bytes)?;
    if bytes.len() as u64 != a.bytes || sha256(&bytes) != a.sha256 {
        return Err(error("artifact size/hash differs"));
    }
    Ok(bytes)
}
fn scan(root: &Path, relative: &Path, depth: usize, files: &mut BTreeSet<String>) -> Result<()> {
    if depth > 4 || files.len() > 32 {
        return Err(error("tree limit"));
    }
    for item in fs::read_dir(root.join(relative))? {
        let item = item?;
        if item.file_type()?.is_symlink() {
            return Err(error("symlink in extension tree"));
        }
        let path = relative.join(item.file_name());
        if item.file_type()?.is_dir() {
            scan(root, &path, depth + 1, files)?;
        } else if item.file_type()?.is_file() {
            files.insert(
                path.to_str()
                    .ok_or_else(|| error("non-UTF8 path"))?
                    .to_string(),
            );
            if files.len() > 32 {
                return Err(error("tree file limit"));
            }
        } else {
            return Err(error("unsupported file type"));
        }
    }
    Ok(())
}
pub fn validate_boundary_input(input: &BoundaryInput) -> Result<()> {
    if input.schema_version != 1
        || !input.synthetic
        || input.records.len() > 64
        || input.updates.len() > 64
        || input.queries.len() + input.after_queries.len() > 64
        || input.measurements.len() > 64
    {
        return Err(error("input schema/bounds"));
    }
    IdentityGraph::from_records(&input.records)?;
    fn bounded_strings(v: &Value) -> bool {
        match v {
            Value::String(s) => s.len() <= 256,
            Value::Array(a) => a.iter().all(bounded_strings),
            Value::Object(m) => m.iter().all(|(k, v)| k.len() <= 256 && bounded_strings(v)),
            _ => true,
        }
    }
    if !bounded_strings(&serde_json::to_value(input).map_err(|_| error("input serialization"))?) {
        return Err(error("string/label bound"));
    }
    let mut ids = BTreeSet::new();
    for q in input.queries.iter().chain(&input.after_queries) {
        let valid = match &q.selector {
            TimeSelector::Unspecified => true,
            TimeSelector::At { clock, .. } => !clock.is_empty(),
            TimeSelector::During {
                start_us,
                end_us,
                clock,
                ..
            } => start_us < end_us && !clock.is_empty(),
        };
        if !valid || q.relation.is_empty() {
            return Err(error("invalid query time/relation"));
        }
        if q.id.is_empty() || !ids.insert(q.id.clone()) {
            return Err(error("duplicate/empty query id"));
        }
    }
    let mut measurements = BTreeSet::new();
    for m in &input.measurements {
        if m.id.is_empty()
            || !measurements.insert(&m.id)
            || !input
                .measurement_queries
                .get(&m.id)
                .is_some_and(|q| input.queries.iter().any(|v| &v.id == q))
        {
            return Err(error("measurement/query reference"));
        }
    }
    if input.measurement_queries.len() != input.measurements.len() {
        return Err(error("extra measurement mapping"));
    }
    Ok(())
}
pub fn load_enrichment_bundle(root: &Path) -> Result<EnrichmentFixtureBundle> {
    let base = fixture::confined(root, Path::new("fixtures/enrichment-v1"))?;
    let bytes = fixture::bounded_read(
        &fixture::confined(&base, Path::new("manifest.json"))?,
        MAX_BYTES,
    )?;
    let manifest: ExtensionManifest =
        serde_json::from_slice(&bytes).map_err(|_| error("manifest schema"))?;
    if manifest.schema_version != 1
        || !manifest.synthetic
        || manifest.bundle_id != "aether-enrichment-fixtures-v1"
        || manifest.cases.len() != 7
    {
        return Err(error("manifest identity/count"));
    }
    for (path, hash) in [
        (
            "fixtures/prototype/manifest.json",
            &manifest.original_manifest_sha256,
        ),
        (
            "metadata-enrichment-matrix.md",
            &manifest.associations_sha256,
        ),
        ("docs/prototype-experiments.json", &manifest.registry_sha256),
    ] {
        if sha256(&fixture::bounded_read(
            &fixture::confined(root, Path::new(path))?,
            MAX_BYTES,
        )?) != *hash
        {
            return Err(error("baseline identity differs"));
        }
    }
    let mut indexed = BTreeSet::from(["manifest.json".into(), "README.md".into()]);
    let mut inputs = BTreeMap::new();
    for c in &manifest.cases {
        if !(1..=7).any(|i| c.case_id == format!("E5-{i:02}")) {
            return Err(error("unknown case id"));
        }
        for a in [&c.input, &c.expected] {
            if !indexed.insert(a.path.clone()) {
                return Err(error("duplicate artifact"));
            }
        }
        let bytes = read_artifact(&base, &c.input, "input")?;
        let input: BoundaryInput =
            serde_json::from_slice(&bytes).map_err(|_| error("input schema"))?;
        if input.case_id != c.case_id || inputs.contains_key(&c.case_id) {
            return Err(error("duplicate/mismatched case"));
        }
        validate_boundary_input(&input)?;
        let _ = read_artifact(&base, &c.expected, "expected")?;
        inputs.insert(c.case_id.clone(), input);
    }
    let mut files = BTreeSet::new();
    scan(&base, Path::new(""), 0, &mut files)?;
    if files != indexed {
        return Err(error("unindexed/missing extension file"));
    }
    Ok(EnrichmentFixtureBundle {
        root: base,
        manifest,
        manifest_sha256: sha256(&bytes),
        inputs,
    })
}
pub fn load_boundary_expected(
    bundle: &EnrichmentFixtureBundle,
    id: &str,
) -> Result<BoundaryExpected> {
    let case = bundle
        .manifest
        .cases
        .iter()
        .find(|c| c.case_id == id)
        .ok_or_else(|| error("unknown case"))?;
    let expected: BoundaryExpected =
        serde_json::from_slice(&read_artifact(&bundle.root, &case.expected, "expected")?)
            .map_err(|_| error("oracle schema"))?;
    if expected.schema_version != 1
        || !expected.synthetic
        || expected.case_id != id
        || expected.queries.len() > 64
    {
        return Err(error("oracle identity/bounds"));
    }
    Ok(expected)
}
