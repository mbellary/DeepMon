use crate::{PrototypeError, Result, config::MANIFEST};
use aether_validation::sha256;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Read,
    path::{Component, Path, PathBuf},
};
pub const MAX_BYTES: u64 = 1024 * 1024;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema_version: u32,
    pub bundle_id: String,
    pub synthetic: bool,
    pub source_manifest_sha256: String,
    pub registry_sha256: String,
    pub status_sha256: String,
    pub environment_id: String,
    pub fixtures: Vec<Group>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
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
    pub variants: Vec<Variant>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub id: String,
    pub purpose: String,
    pub intent: String,
    pub expected_payload_error: Option<String>,
    pub experiment_execution_status: String,
    pub artifacts: Vec<Artifact>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub path: String,
    pub role: String,
    pub media_type: String,
    pub bytes: u64,
    pub sha256: String,
}
#[derive(Debug)]
pub struct ValidatedBundle {
    pub manifest: Manifest,
    pub manifest_sha256: String,
    pub root: PathBuf,
    pub counts: [usize; 4],
}
/// Endpoint input contains no semantic oracle. Fields cannot be forged by callers.
#[derive(Clone, Debug)]
pub struct EndpointInput {
    pub(crate) variant_id: String,
    pub(crate) bundle_id: String,
    pub(crate) manifest_sha256: String,
    pub(crate) expected_hash: String,
    pub(crate) bytes: Vec<u8>,
}
impl EndpointInput {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn variant_id(&self) -> &str {
        &self.variant_id
    }
}
fn confined(root: &Path, relative: &Path) -> Result<PathBuf> {
    let base = root.canonicalize()?;
    if relative.as_os_str().is_empty()
        || !relative
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
    {
        return Err(PrototypeError::new("unsafe_path", relative.display()));
    }
    let mut path = base.clone();
    for c in relative.components() {
        path.push(c);
        if fs::symlink_metadata(&path)?.file_type().is_symlink() {
            return Err(PrototypeError::new(
                "unsafe_path",
                "symlinks are not accepted",
            ));
        }
    }
    if !path.canonicalize()?.starts_with(base) {
        return Err(PrototypeError::new(
            "unsafe_path",
            "path escapes fixture root",
        ));
    }
    Ok(path)
}
fn bounded_read(path: &Path, limit: u64) -> Result<Vec<u8>> {
    if limit > MAX_BYTES || fs::metadata(path)?.len() > limit {
        return Err(PrototypeError::new("size_limit", path.display()));
    }
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take(limit + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(PrototypeError::new("size_limit", path.display()));
    }
    Ok(bytes)
}
pub fn load_bundle(root: &Path, manifest: &Path) -> Result<ValidatedBundle> {
    if manifest != Path::new(MANIFEST) {
        return Err(PrototypeError::new(
            "unsupported_manifest",
            manifest.display(),
        ));
    }
    let root = root.canonicalize()?;
    let manifest_path = confined(&root, manifest)?;
    let original = bounded_read(&manifest_path, MAX_BYTES)?;
    let summary = aether_validation::check_fixtures(&root).map_err(|errors| {
        PrototypeError::new(
            "fixture_validation",
            errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; "),
        )
    })?;
    // The full checker is authoritative. Re-read and reject a concurrent manifest change.
    if bounded_read(&manifest_path, MAX_BYTES)? != original {
        return Err(PrototypeError::new(
            "input_changed",
            "manifest changed during validation",
        ));
    }
    let mut parsed: Manifest = serde_json::from_slice(&original)
        .map_err(|e| PrototypeError::new("manifest_invalid", e))?;
    let fixture_root = confined(&root, Path::new("fixtures/prototype"))?;
    for group in &mut parsed.fixtures {
        group.variants.sort_by(|a, b| a.id.cmp(&b.id));
        for variant in &group.variants {
            for artifact in &variant.artifacts {
                let _ = confined(&fixture_root, Path::new(&artifact.path))?;
            }
        }
    }
    parsed
        .fixtures
        .sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    let variants: usize = parsed.fixtures.iter().map(|g| g.variants.len()).sum();
    if parsed.fixtures.len() != summary.groups || variants != summary.variants {
        return Err(PrototypeError::new(
            "input_changed",
            "summary/index mismatch",
        ));
    }
    Ok(ValidatedBundle {
        manifest: parsed,
        manifest_sha256: sha256(&original),
        root,
        counts: [
            summary.groups,
            summary.variants,
            summary.files,
            summary.invalid_payloads,
        ],
    })
}
pub fn select_endpoint(bundle: &ValidatedBundle, variant_id: &str) -> Result<EndpointInput> {
    let variant = bundle
        .manifest
        .fixtures
        .iter()
        .flat_map(|g| &g.variants)
        .find(|v| v.id == variant_id)
        .ok_or_else(|| {
            PrototypeError::new("variant_unknown", "no such fixture variant").fixture(variant_id)
        })?;
    let endpoints: Vec<_> = variant
        .artifacts
        .iter()
        .filter(|a| a.role == "endpoint")
        .collect();
    if endpoints.len() != 1 {
        return Err(PrototypeError::new(
            "endpoint_unavailable",
            "variant must have exactly one endpoint",
        )
        .fixture(variant_id));
    }
    let a = endpoints[0];
    let base = confined(&bundle.root, Path::new("fixtures/prototype"))?;
    let bytes = bounded_read(&confined(&base, Path::new(&a.path))?, a.bytes)?;
    if bytes.len() as u64 != a.bytes || sha256(&bytes) != a.sha256 {
        return Err(
            PrototypeError::new("artifact_changed", "endpoint length/hash mismatch")
                .fixture(variant_id),
        );
    }
    Ok(EndpointInput {
        variant_id: variant_id.into(),
        bundle_id: bundle.manifest.bundle_id.clone(),
        manifest_sha256: bundle.manifest_sha256.clone(),
        expected_hash: a.sha256.clone(),
        bytes,
    })
}
