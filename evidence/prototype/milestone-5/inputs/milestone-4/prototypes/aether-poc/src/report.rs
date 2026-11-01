use crate::fixture::{Group, ValidatedBundle};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct FixtureListing<'a> {
    pub schema_version: u32,
    pub synthetic: bool,
    pub bundle_id: &'a str,
    pub manifest_sha256: &'a str,
    pub fixture_groups: usize,
    pub fixture_variants: usize,
    pub fixture_payload_files: usize,
    pub intentional_invalid_payloads: usize,
    pub construction_validation: &'static str,
    pub harness_load: &'static str,
    pub semantic_execution_status: &'static str,
    pub runtime_acceptance_readiness: bool,
    pub groups: Option<&'a [Group]>,
}
impl<'a> FixtureListing<'a> {
    pub fn new(bundle: &'a ValidatedBundle, list: bool) -> Self {
        Self {
            schema_version: 1,
            synthetic: true,
            bundle_id: &bundle.manifest.bundle_id,
            manifest_sha256: &bundle.manifest_sha256,
            fixture_groups: bundle.counts[0],
            fixture_variants: bundle.counts[1],
            fixture_payload_files: bundle.counts[2],
            intentional_invalid_payloads: bundle.counts[3],
            construction_validation: "passed",
            harness_load: "passed",
            semantic_execution_status: "not_run",
            runtime_acceptance_readiness: false,
            groups: list.then_some(bundle.manifest.fixtures.as_slice()),
        }
    }
}
#[derive(Debug, Serialize)]
pub struct ScenarioReport {
    pub schema_version: u32,
    pub synthetic: bool,
    pub scenario: &'static str,
    pub variant_id: String,
    pub bundle_id: String,
    pub manifest_sha256: String,
    pub expected_bytes: usize,
    pub actual_bytes: usize,
    pub expected_sha256: String,
    pub actual_sha256: String,
    pub transport_status: &'static str,
    pub harness_status: &'static str,
    pub semantic_execution_status: &'static str,
    pub runtime_acceptance_readiness: bool,
    pub production_support_claimed: bool,
}
