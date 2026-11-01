use crate::{
    PrototypeError, Result,
    enrichment_fixture::*,
    fixture::{self, ValidatedBundle},
    identity::*,
    metric_input::{self, RecordKind, ScenarioInput},
    observation::{self, DerivedResult, QualityState},
    scenario::{self, MetricScenario},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, str::FromStr};
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EnrichmentScenario {
    PublisherIdentity,
    ProcessLifecycle,
    ResourceMembership,
    FabricContext,
    VisibilityEnrichment,
}
impl EnrichmentScenario {
    pub fn group(self) -> &'static str {
        match self {
            Self::PublisherIdentity => "PX04",
            Self::ProcessLifecycle => "PX06",
            Self::ResourceMembership => "PX05",
            Self::FabricContext => "PX07",
            Self::VisibilityEnrichment => "PX11",
        }
    }
    pub fn for_group(g: &str) -> Option<Self> {
        [
            Self::PublisherIdentity,
            Self::ProcessLifecycle,
            Self::ResourceMembership,
            Self::FabricContext,
            Self::VisibilityEnrichment,
        ]
        .into_iter()
        .find(|s| s.group() == g)
    }
}
impl FromStr for EnrichmentScenario {
    type Err = PrototypeError;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "publisher-identity" => Ok(Self::PublisherIdentity),
            "process-lifecycle" => Ok(Self::ProcessLifecycle),
            "resource-membership" => Ok(Self::ResourceMembership),
            "fabric-context" => Ok(Self::FabricContext),
            "visibility-enrichment" => Ok(Self::VisibilityEnrichment),
            _ => Err(PrototypeError::new(
                "not_prototyped",
                "unimplemented enrichment scenario",
            )),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnrichmentReport {
    pub schema_version: u32,
    pub synthetic: bool,
    pub scenario: String,
    pub input_id: String,
    pub manifest_sha256: String,
    pub input_sha256: String,
    pub functional_execution_status: String,
    pub results: Vec<DerivedResult>,
    pub associations: BTreeMap<String, AssociationResult>,
    pub observations: Vec<EnrichedObservation>,
    pub context_facts: Vec<Value>,
    pub measurement_count: usize,
    pub update_error: Option<String>,
    pub history_unchanged: bool,
    pub permutation_invariant: bool,
    pub physical_identity_status: String,
    pub px_runtime_acceptance_status: String,
    pub real_verification: String,
    pub runtime_acceptance_readiness: bool,
    pub production_support_claimed: bool,
}
fn error() -> PrototypeError {
    PrototypeError::new(
        "enrichment_input_invalid",
        "invalid scoped fixture metadata; payload omitted",
    )
}
fn safe(report: EnrichmentReport) -> Result<EnrichmentReport> {
    let mut v = serde_json::to_value(report).map_err(|_| error())?;
    observation::sanitize(&mut v);
    serde_json::from_value(v).map_err(|_| error())
}
fn report(id: &str, scenario: &str, manifest: &str, input: &str) -> EnrichmentReport {
    EnrichmentReport {
        schema_version: 1,
        synthetic: true,
        scenario: scenario.into(),
        input_id: id.into(),
        manifest_sha256: manifest.into(),
        input_sha256: input.into(),
        functional_execution_status: "executed".into(),
        results: vec![],
        associations: BTreeMap::new(),
        observations: vec![],
        context_facts: vec![],
        measurement_count: 0,
        update_error: None,
        history_unchanged: true,
        permutation_invariant: true,
        physical_identity_status: "unverified_synthetic".into(),
        px_runtime_acceptance_status: "blocked_unagreed_budget".into(),
        real_verification: "not_run".into(),
        runtime_acceptance_readiness: false,
        production_support_claimed: false,
    }
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct IdentityInput {
    entity: String,
    incarnation: String,
    valid_from_us: u64,
    valid_to_us: u64,
    evidence: String,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublisherInput {
    publisher_candidates: Vec<String>,
    model_alias: String,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MembershipInput {
    process_record: String,
    container: String,
    cgroup: String,
    valid_from_us: u64,
    valid_to_us: u64,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DeniedInput {
    container: Option<String>,
    source: String,
    status: String,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MembersInput {
    replica: String,
    members: Vec<String>,
    complete: bool,
    models: Option<Vec<String>>,
    other_replica_same_model_alias: Option<String>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilityInput {
    capabilities: Vec<String>,
    configured_preference: String,
    driver: String,
    loaded_cuda_nccl: Option<Value>,
    model: String,
    operation_path: Option<Value>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ControlInput {
    protocol: String,
    socket: String,
    is_payload_path: Option<bool>,
}
fn key(scope: &str, kind: &str, locator: &str, incarnation: &str) -> EntityKey {
    EntityKey {
        scope: scope.into(),
        kind: kind.into(),
        locator: locator.into(),
        incarnation: incarnation.into(),
        host: None,
        boot: None,
        namespace: None,
    }
}
fn single_clock(input: &ScenarioInput) -> Result<&metric_input::Clock> {
    if input.clocks.len() != 1 || input.clocks[0].unit != "microseconds" {
        return Err(error());
    }
    Ok(&input.clocks[0])
}
fn timed(input: &ScenarioInput, start: u64, end: u64) -> Result<Option<Validity>> {
    let clock = single_clock(input)?;
    if start >= end {
        return Err(error());
    }
    Ok(Some(Validity {
        start_us: start,
        end_us: end,
        clock: clock.domain.clone(),
        uncertainty_us: clock.uncertainty_us,
    }))
}
fn identities(
    input: &ScenarioInput,
    kind: &str,
) -> Result<(Vec<MetadataRecord>, BTreeMap<String, EntityKey>)> {
    let mut records = vec![];
    let mut keys = BTreeMap::new();
    for r in &input.records {
        if matches!(r.kind, RecordKind::Identity) {
            let id: IdentityInput = r.decode()?;
            let k = key(&input.variant_id, kind, &id.entity, &id.incarnation);
            records.push(MetadataRecord::Node {
                node: EntityNode {
                    id: r.id.clone(),
                    key: k.clone(),
                    validity: timed(input, id.valid_from_us, id.valid_to_us)?,
                    evidence: vec![r.id.clone(), id.evidence],
                },
            });
            keys.insert(r.id.clone(), k);
        }
    }
    Ok((records, keys))
}
fn relation(
    id: &str,
    from: &EntityKey,
    to: Vec<EntityRef>,
    kind: &str,
    validity: Option<Validity>,
    complete: bool,
    denied: bool,
) -> MetadataRecord {
    MetadataRecord::Relation {
        relation: Relationship {
            id: id.into(),
            from: from.clone(),
            to,
            kind: kind.into(),
            validity,
            complete,
            evidence: vec![id.into()],
            denied,
        },
    }
}
fn query(id: &str, from: &EntityKey, kind: &str, is_set: bool) -> AssociationQuery {
    AssociationQuery {
        id: id.into(),
        from: from.clone(),
        relation: kind.into(),
        selector: TimeSelector::Unspecified,
        is_set,
    }
}
fn symbolic(scope: &str, locator: &str) -> EntityRef {
    EntityRef::Symbolic {
        scope: scope.into(),
        locator: locator.into(),
    }
}
fn association_result(
    target: &str,
    result: &AssociationResult,
    unit: &str,
    value: Value,
) -> DerivedResult {
    derive_state(
        target,
        result.state,
        unit,
        value,
        &result.reason_code,
        result.evidence.clone(),
    )
}
fn derive_state(
    target: &str,
    state: QualityState,
    unit: &str,
    value: Value,
    reason: &str,
    ids: Vec<String>,
) -> DerivedResult {
    crate::derive::state(target, value, unit, state, reason, ids)
}
fn absent() -> EntityRef {
    EntityRef::Unknown {
        reason: "publisher_evidence_missing".into(),
    }
}
pub fn run_enrichment_scenario(
    bundle: &ValidatedBundle,
    scenario: EnrichmentScenario,
    variant_id: &str,
) -> Result<EnrichmentReport> {
    let group = bundle
        .manifest
        .fixtures
        .iter()
        .find(|g| g.variants.iter().any(|v| v.id == variant_id))
        .ok_or_else(|| PrototypeError::new("variant_unknown", "unknown variant"))?;
    if scenario.group() != group.experiment_id {
        return Err(PrototypeError::new(
            "scenario_incompatible",
            "scenario/group mismatch",
        ));
    }
    let input = fixture::load_scenario_input(bundle, variant_id)?;
    let mut out = report(
        variant_id,
        serde_json::to_value(scenario)
            .map_err(|_| error())?
            .as_str()
            .ok_or_else(error)?,
        &bundle.manifest_sha256,
        &input.sha256,
    );
    match scenario {
        EnrichmentScenario::PublisherIdentity => {
            let (mut records, keys) = identities(&input, "publisher_candidate")?;
            if let Some(r) = input
                .records
                .iter()
                .find(|r| r.get("publisher_candidates").is_some())
            {
                let listener: PublisherInput = r.decode()?;
                let from = key(
                    variant_id,
                    "fixture_link",
                    &r.id,
                    &format!("record:{}", r.id),
                );
                records.push(MetadataRecord::Node {
                    node: EntityNode {
                        id: format!("context:{}", r.id),
                        key: from.clone(),
                        validity: None,
                        evidence: vec![r.id.clone()],
                    },
                });
                let targets: Vec<_> = listener
                    .publisher_candidates
                    .iter()
                    .map(|id| {
                        keys.get(id)
                            .cloned()
                            .map(|key| EntityRef::Key { key })
                            .ok_or_else(error)
                    })
                    .collect::<Result<_>>()?;
                records.push(relation(
                    &r.id,
                    &from,
                    targets,
                    "publisher",
                    None,
                    true,
                    false,
                ));
                let graph = IdentityGraph::from_records(&records)?;
                let resolution =
                    resolve_association(&graph, &query("publisher", &from, "publisher", false));
                let mut candidate_ids: Vec<_> = resolution
                    .candidates
                    .iter()
                    .filter_map(|r| {
                        if let EntityRef::Key { key } = r {
                            keys.iter()
                                .find(|(_, k)| *k == key)
                                .map(|(id, _)| id.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                candidate_ids.sort();
                out.results.push(association_result(
                    "publisher",
                    &resolution,
                    "candidate_set",
                    json!(candidate_ids),
                ));
                out.associations.insert("publisher".into(), resolution);
                out.context_facts.push(json!({"reported_model_alias":listener.model_alias,"ownership_from_alias":false}));
            } else {
                let prior = scenario::run_metric_scenario(
                    bundle,
                    MetricScenario::CountBoundary,
                    variant_id,
                )?;
                out.results = prior.results;
                for r in &input.records {
                    if r.get("subject").is_some() {
                        out.context_facts.push(json!({"record_id":r.id,"emitter":r.get("emitter"),"subject":r.get("subject"),"subject_host":null,"temporal_status":"unspecified_input"}));
                    }
                }
                for r in &input.records {
                    if matches!(r.kind, RecordKind::Aggregate) {
                        out.context_facts.push(json!({"raw_count_record":r.id,"producer":r.get("producer"),"count":r.get("count")}));
                    }
                }
            }
        }
        EnrichmentScenario::ProcessLifecycle => {
            let (mut records, keys) = identities(&input, "process")?;
            for r in &input.records {
                if r.get("process_record").is_some() {
                    let membership: MembershipInput = r.decode()?;
                    let from = keys.get(&membership.process_record).ok_or_else(error)?;
                    let validity = timed(&input, membership.valid_from_us, membership.valid_to_us)?;
                    records.push(relation(
                        &r.id,
                        from,
                        vec![symbolic(variant_id, &membership.container)],
                        "container",
                        validity.clone(),
                        true,
                        false,
                    ));
                    records.push(relation(
                        &format!("{}:cgroup", r.id),
                        from,
                        vec![symbolic(variant_id, &membership.cgroup)],
                        "cgroup",
                        validity,
                        true,
                        false,
                    ));
                } else if matches!(r.kind, RecordKind::Policy) {
                    let denied: DeniedInput = r.decode()?;
                    if denied.container.is_some()
                        || denied.status != "permission_denied"
                        || denied.source != "runtime_metadata"
                    {
                        return Err(error());
                    }
                    let from = keys.values().next().ok_or_else(error)?;
                    if keys.len() != 1 {
                        return Err(error());
                    }
                    records.push(relation(
                        &r.id,
                        from,
                        vec![],
                        "container",
                        None,
                        false,
                        true,
                    ));
                    out.context_facts.push(json!({"source":denied.source,"status":denied.status,"known_processes":keys.values().collect::<Vec<_>>()}));
                }
            }
            let graph = IdentityGraph::from_records(&records)?;
            for (id, k) in &keys {
                let mut q = query(&format!("owner:{id}"), k, "container", false);
                let node = records
                    .iter()
                    .find_map(|r| {
                        if let MetadataRecord::Node { node } = r {
                            if &node.key == k { Some(node) } else { None }
                        } else {
                            None
                        }
                    })
                    .ok_or_else(error)?;
                if input
                    .records
                    .iter()
                    .any(|r| r.get("process_record") == Some(&json!(id)))
                {
                    let v = node.validity.as_ref().ok_or_else(error)?;
                    q.selector = TimeSelector::During {
                        start_us: v.start_us,
                        end_us: v.end_us,
                        clock: v.clock.clone(),
                        uncertainty_us: v.uncertainty_us,
                    };
                }
                let resolution = resolve_association(&graph, &q);
                let target = if id == "old" {
                    "historical_owner"
                } else if id == "new" {
                    "new_owner"
                } else {
                    "container"
                };
                if resolution.state == QualityState::Resolved && resolution.candidates.len() == 1 {
                    out.results.push(association_result(
                        target,
                        &resolution,
                        "container",
                        json!(resolution.candidates[0].locator()),
                    ));
                } else {
                    out.results
                        .push(DerivedResult::unknown(target, &resolution.reason_code));
                }
                out.associations.insert(target.into(), resolution);
            }
        }
        EnrichmentScenario::ResourceMembership => {
            let (mut records, keys) = identities(&input, "replica")?;
            let r = input
                .records
                .iter()
                .find(|r| r.get("members").is_some())
                .ok_or_else(error)?;
            let members: MembersInput = r.decode()?;
            if members
                .models
                .as_ref()
                .is_some_and(|m| m.len() != members.members.len())
            {
                return Err(error());
            }
            let actual_key = keys
                .values()
                .find(|k| k.locator == members.replica)
                .cloned();
            let from = actual_key.clone().unwrap_or_else(|| {
                key(
                    variant_id,
                    "declared_membership",
                    &r.id,
                    &format!("record:{}", r.id),
                )
            });
            if actual_key.is_none() {
                records.push(MetadataRecord::Node {
                    node: EntityNode {
                        id: format!("context:{}", r.id),
                        key: from.clone(),
                        validity: None,
                        evidence: vec![r.id.clone()],
                    },
                });
            }
            records.push(relation(
                &r.id,
                &from,
                members
                    .members
                    .iter()
                    .map(|v| symbolic(variant_id, v))
                    .collect(),
                "allocated_resources",
                None,
                members.complete,
                false,
            ));
            let graph = IdentityGraph::from_records(&records)?;
            let q = query("members", &from, "allocated_resources", true);
            let resolution = resolve_association(&graph, &q);
            out.results =
                scenario::run_metric_scenario(bundle, MetricScenario::MemoryOccupancy, variant_id)?
                    .results;
            if !members.complete {
                out.results.push(association_result(
                    "composition",
                    &resolution,
                    "device_set",
                    json!(
                        resolution
                            .candidates
                            .iter()
                            .map(EntityRef::locator)
                            .collect::<Vec<_>>()
                    ),
                ));
            }
            out.context_facts.push(json!({"declared_replica":members.replica,"models":members.models,"other_replica_same_model_alias":members.other_replica_same_model_alias,"resolved_replica_identity":actual_key,"observed_participants":null,"physical_devices_verified":false}));
            for r in &input.records {
                if matches!(r.kind, RecordKind::Aggregate) {
                    let count: u64 = r.get("count").and_then(Value::as_u64).ok_or_else(error)?;
                    let measurement = EnrichmentInputMeasurement {
                        id: format!("{variant_id}:{}", r.id),
                        native: json!({"record_id":r.id,"kind":"aggregate","count":count,"scope":r.get("scope")}),
                        emitter: absent(),
                        subject: actual_key.clone().map(|key| EntityRef::Key { key }),
                        value: json!(count),
                        unit: "requests".into(),
                        scope: members.replica.clone(),
                    };
                    out.observations
                        .push(associate_observation(&measurement, &graph, &q)?);
                }
            }
            out.associations
                .insert("allocated_resources".into(), resolution);
            out.associations.insert(
                "observed_participants".into(),
                resolve_association(
                    &graph,
                    &query("participants", &from, "observed_participants", true),
                ),
            );
        }
        EnrichmentScenario::FabricContext => {
            let (records, keys) = identities(&input, "nic")?;
            let graph = IdentityGraph::from_records(&records)?;
            for r in &input.records {
                if r.get("capabilities").is_some() {
                    let cap: CapabilityInput = r.decode()?;
                    if cap.operation_path.is_some() || cap.loaded_cuda_nccl.is_some() {
                        return Err(PrototypeError::new(
                            "not_prototyped",
                            "operation/version attribution needs deeper explicit evidence",
                        ));
                    }
                    out.results.push(derive_state(
                        "capabilities",
                        QualityState::ReportedOnly,
                        "capability_set",
                        json!(cap.capabilities),
                        "reported_unattached_capability",
                        vec![r.id.clone()],
                    ));
                    out.results.push(DerivedResult::unknown(
                        "active_transport",
                        "payload_path_evidence_missing",
                    ));
                    out.context_facts.push(json!({"record_id":r.id,"reported_model":cap.model,"reported_driver":cap.driver,"configured_preference":cap.configured_preference,"nic_foreign_key":null,"loaded_cuda_nccl":null,"physical_identity_verified":false}));
                } else if r.get("protocol").is_some() {
                    let control: ControlInput = r.decode()?;
                    out.results.push(DerivedResult::unknown(
                        "operation_to_nic",
                        "control_socket_not_payload_evidence",
                    ));
                    out.context_facts.push(json!({"record_id":r.id,"protocol":control.protocol,"socket":control.socket,"is_payload_path":control.is_payload_path,"resolved_peer":null}));
                } else if matches!(r.kind, RecordKind::Gauge) {
                    let gauge: FabricGauge = r.decode()?;
                    if !gauge
                        .value
                        .as_f64()
                        .is_some_and(|n| n.is_finite() && n >= 0.0)
                    {
                        return Err(error());
                    }
                    out.context_facts.push(json!({"record_id":r.id,"value":gauge.value,"unit":gauge.unit,"reported_scope":gauge.scope,"capture_time":null}));
                }
            }
            out.context_facts
                .push(json!({"nic_identities":keys.values().collect::<Vec<_>>() }));
            for key in keys.values() {
                out.associations.insert(
                    "active_transport".into(),
                    resolve_association(&graph, &query("path", key, "active_transport", false)),
                );
            }
        }
        EnrichmentScenario::VisibilityEnrichment => {
            let prior = scenario::run_metric_scenario(
                bundle,
                MetricScenario::VisibilityPolicy,
                variant_id,
            )?;
            out.results = prior.results;
            out.context_facts = prior.reported_input_records;
        }
    }
    out.results.sort_by(|a, b| a.target.cmp(&b.target));
    out.measurement_count = out.observations.len();
    safe(out)
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FabricGauge {
    scope: String,
    unit: String,
    value: serde_json::Number,
}
#[derive(Debug, Serialize)]
pub struct EnrichmentCheck {
    pub assigned_milestone: u32,
    pub functional_execution_status: String,
    pub assertion_id: String,
    pub target: String,
    pub functional_check_status: String,
    pub newly_exercised_in_m5: bool,
    pub evidence_kind: String,
}
#[derive(Debug, Serialize)]
pub struct EnrichmentVerification {
    pub variant_id: String,
    pub input_sha256: String,
    pub oracle_sha256: String,
    pub result_sha256: String,
    pub actual: EnrichmentReport,
    pub checks: Vec<EnrichmentCheck>,
    pub passed: bool,
}
fn fingerprint(value: &impl Serialize) -> Result<String> {
    let mut value = serde_json::to_value(value).map_err(|_| error())?;
    observation::sanitize(&mut value);
    Ok(aether_validation::sha256(
        &serde_json::to_vec(&value).map_err(|_| error())?,
    ))
}
pub fn verify_enrichment_scenario(
    bundle: &ValidatedBundle,
    scenario: EnrichmentScenario,
    variant_id: &str,
) -> Result<EnrichmentVerification> {
    let actual = run_enrichment_scenario(bundle, scenario, variant_id)?;
    let expected = fixture::load_expected(bundle, variant_id)?;
    let mut checks = vec![];
    for (index, e) in expected.assertions.iter().enumerate() {
        let matched = actual
            .results
            .iter()
            .any(|r| scenario::compare_result(r, e));
        let new = scenario == EnrichmentScenario::ProcessLifecycle
            || scenario == EnrichmentScenario::FabricContext
            || e.target == "publisher"
            || e.target == "composition";
        checks.push(EnrichmentCheck {
            assigned_milestone: if new { 5 } else { 4 },
            functional_execution_status: "executed_in_m5".into(),
            assertion_id: format!("{variant_id}:{index}:{}:{}", e.kind, e.target),
            target: e.target.clone(),
            functional_check_status: if matched { "matched" } else { "mismatched" }.into(),
            newly_exercised_in_m5: new,
            evidence_kind: "synthetic_function_check".into(),
        });
    }
    let passed = checks
        .iter()
        .all(|c| c.functional_check_status == "matched");
    Ok(EnrichmentVerification {
        variant_id: variant_id.into(),
        input_sha256: actual.input_sha256.clone(),
        oracle_sha256: expected.sha256,
        result_sha256: fingerprint(&actual)?,
        actual,
        checks,
        passed,
    })
}
#[derive(Debug, Serialize)]
pub struct BoundaryReport {
    pub actual: EnrichmentReport,
    pub queries: BTreeMap<String, AssociationResult>,
}
pub fn run_boundary_case(
    bundle: &EnrichmentFixtureBundle,
    case_id: &str,
) -> Result<BoundaryReport> {
    let input = bundle
        .inputs
        .get(case_id)
        .ok_or_else(|| PrototypeError::new("case_unknown", "unknown boundary case"))?;
    validate_boundary_input(input)?;
    let graph = IdentityGraph::from_records(&input.records)?;
    let mut queries = BTreeMap::new();
    for q in &input.queries {
        queries.insert(q.id.clone(), resolve_association(&graph, q));
    }
    let index = bundle
        .manifest
        .cases
        .iter()
        .find(|c| c.case_id == case_id)
        .ok_or_else(error)?;
    let mut actual = report(
        case_id,
        "enrichment-boundaries",
        &bundle.manifest_sha256,
        &index.input.sha256,
    );
    for m in &input.measurements {
        let qid = &input.measurement_queries[&m.id];
        let q = input
            .queries
            .iter()
            .find(|q| &q.id == qid)
            .ok_or_else(error)?;
        actual
            .observations
            .push(associate_observation(m, &graph, q)?);
    }
    let history = actual.observations.clone();
    let updated = match graph.with_records(&input.updates) {
        Ok(g) => g,
        Err(e) => {
            actual.update_error = Some(e.code.into());
            graph.clone()
        }
    };
    for q in &input.after_queries {
        queries.insert(q.id.clone(), resolve_association(&updated, q));
    }
    actual.history_unchanged = actual.observations == history;
    let mut reverse = input.records.clone();
    reverse.reverse();
    let reversed = IdentityGraph::from_records(&reverse)?;
    actual.permutation_invariant = input
        .queries
        .iter()
        .all(|q| resolve_association(&reversed, q) == resolve_association(&graph, q));
    actual.measurement_count = actual.observations.len();
    actual.associations = queries.clone();
    let actual = safe(actual)?;
    let queries = actual.associations.clone();
    Ok(BoundaryReport { actual, queries })
}
#[derive(Debug, Serialize)]
pub struct BoundaryVerification {
    pub case_id: String,
    pub oracle_sha256: String,
    pub result_sha256: String,
    pub actual: BoundaryReport,
    pub checks: BTreeMap<String, bool>,
    pub passed: bool,
}
pub fn verify_boundary_case(
    bundle: &EnrichmentFixtureBundle,
    case_id: &str,
) -> Result<BoundaryVerification> {
    let actual = run_boundary_case(bundle, case_id)?;
    let expected = load_boundary_expected(bundle, case_id)?;
    let mut checks = BTreeMap::new();
    for (id, e) in &expected.queries {
        let matched = actual.queries.get(id).is_some_and(|r| {
            r.state == e.state
                && r.candidates
                    .iter()
                    .map(EntityRef::locator)
                    .collect::<Vec<_>>()
                    == e.candidates.iter().map(String::as_str).collect::<Vec<_>>()
                && e.temporal_status
                    .as_ref()
                    .is_none_or(|t| t == &r.temporal_status)
        });
        checks.insert(format!("query:{id}"), matched);
    }
    checks.insert(
        "query_coverage".into(),
        actual.queries.len() == expected.queries.len(),
    );
    let values: BTreeMap<_, _> = actual
        .actual
        .observations
        .iter()
        .map(|m| (m.measurement.id.clone(), m.measurement.value.clone()))
        .collect();
    checks.insert(
        "measurement_identity_value_cardinality".into(),
        values == expected.measurement_values && actual.actual.measurement_count == values.len(),
    );
    let input = &bundle.inputs[case_id];
    let mut original = serde_json::to_value(&input.measurements).map_err(|_| error())?;
    observation::sanitize(&mut original);
    let actual_measurements: Vec<_> = actual
        .actual
        .observations
        .iter()
        .map(|o| &o.measurement)
        .collect();
    checks.insert(
        "native_emitter_subject_scope_preserved".into(),
        serde_json::to_value(actual_measurements).map_err(|_| error())? == original,
    );
    checks.insert(
        "history_unchanged".into(),
        actual.actual.history_unchanged == expected.history_unchanged,
    );
    checks.insert(
        "permutation_invariant".into(),
        actual.actual.permutation_invariant == expected.permutation_invariant,
    );
    checks.insert(
        "update_error".into(),
        actual.actual.update_error == expected.update_error,
    );
    let passed = checks.values().all(|v| *v);
    let index = bundle
        .manifest
        .cases
        .iter()
        .find(|c| c.case_id == case_id)
        .ok_or_else(error)?;
    Ok(BoundaryVerification {
        case_id: case_id.into(),
        oracle_sha256: index.expected.sha256.clone(),
        result_sha256: fingerprint(&actual)?,
        actual,
        checks,
        passed,
    })
}
pub fn verify_enrichment_milestone(
    bundle: &ValidatedBundle,
    extension: &EnrichmentFixtureBundle,
) -> Result<Value> {
    let mut groups = vec![];
    let mut matched = 0;
    let mut newly = 0;
    let mut original_ids = std::collections::BTreeSet::new();
    let mut passed = true;
    let predecessor_bytes = std::fs::read(
        bundle
            .root
            .join("docs/prototype-metric-execution-status.json"),
    )?;
    let prior_hash = aether_validation::sha256(&predecessor_bytes);
    let accepted = std::fs::read_to_string(
        bundle
            .root
            .join("evidence/prototype/milestone-4/artifacts.sha256"),
    )?;
    if !accepted
        .lines()
        .any(|line| line == format!("{prior_hash}  docs/prototype-metric-execution-status.json"))
    {
        return Err(PrototypeError::new(
            "predecessor_invalid",
            "M4 ledger hash does not match accepted artifact index",
        ));
    }
    let predecessor: Value = serde_json::from_slice(&predecessor_bytes).map_err(|_| error())?;
    if predecessor["manifest_sha256"] != bundle.manifest_sha256
        || predecessor["functional_checks_passed"] != true
    {
        return Err(PrototypeError::new(
            "predecessor_invalid",
            "M4 ledger identity/verdict",
        ));
    }
    let mut combined: BTreeMap<String, usize> = BTreeMap::new();
    for group in &bundle.manifest.fixtures {
        let mut variants = vec![];
        for v in &group.variants {
            if let Some(s) = EnrichmentScenario::for_group(&group.experiment_id) {
                let result = verify_enrichment_scenario(bundle, s, &v.id)?;
                passed &= result.passed;
                for c in &result.checks {
                    *combined
                        .entry(c.functional_check_status.clone())
                        .or_default() += 1;
                    if !original_ids.insert(c.assertion_id.clone()) {
                        return Err(error());
                    }
                    if c.functional_check_status == "matched" {
                        matched += 1;
                        if c.newly_exercised_in_m5 {
                            newly += 1;
                        }
                    }
                }
                variants.push(serde_json::to_value(result).map_err(|_| error())?);
            } else {
                let prior = predecessor["groups"]
                    .as_array()
                    .and_then(|gs| {
                        gs.iter()
                            .find(|g| g["experiment_id"] == group.experiment_id)
                    })
                    .and_then(|g| g["variants"].as_array())
                    .and_then(|vs| vs.iter().find(|p| p["variant_id"] == v.id))
                    .ok_or_else(error)?;
                let e = fixture::load_expected(bundle, &v.id)?;
                let checks = prior["checks"].as_array().ok_or_else(error)?;
                if checks.len() != e.assertions.len() || prior["oracle_sha256"] != e.sha256 {
                    return Err(error());
                }
                for (index, a) in e.assertions.iter().enumerate() {
                    let status = if ["PX08", "PX09", "PX10"].contains(&group.experiment_id.as_str())
                    {
                        "not_run_later_stage"
                    } else {
                        checks[index]["functional_check_status"]
                            .as_str()
                            .ok_or_else(error)?
                    };
                    *combined.entry(status.into()).or_default() += 1;
                    let id = format!("{}:{index}:{}:{}", v.id, a.kind, a.target);
                    if checks[index]["assertion_id"] != id || !original_ids.insert(id) {
                        return Err(error());
                    }
                }
                variants.push(json!({"variant_id":v.id,"functional_execution_status":"not_rerun_in_m5","prior_evidence":"docs/prototype-metric-execution-status.json","oracle_sha256":e.sha256,"checks":checks}));
            }
        }
        groups.push(json!({"experiment_id":group.experiment_id,"variants":variants}));
    }
    let mut boundaries = vec![];
    for id in extension.inputs.keys() {
        let v = verify_boundary_case(extension, id)?;
        passed &= v.passed;
        boundaries.push(v);
    }
    if original_ids.len() != 49 || matched != 17 || newly != 7 {
        passed = false;
    }
    let mut value = json!({"schema_version":1,"synthetic":true,"milestone":5,"manifest_sha256":bundle.manifest_sha256,"extension_manifest_sha256":extension.manifest_sha256,"prior_ledger_sha256":prior_hash,"functional_checks_passed":passed,"original_assertions_indexed":original_ids.len(),"current_original_matches":matched,"new_original_matches":newly,"prior_matches_rechecked":matched-newly,"combined_original_assertion_outcomes":combined,"groups":groups,"boundary_cases":boundaries,"runtime_acceptance_readiness":false,"px_runtime_acceptance_status":"blocked_unagreed_budget","real_verification":"not_run","production_support_claimed":false});
    observation::sanitize(&mut value);
    Ok(value)
}
