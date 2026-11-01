//! Pure synthetic identity resolution. No discovery or current-owner mutation.
use crate::{PrototypeError, Result, observation::QualityState};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityKey {
    pub scope: String,
    pub kind: String,
    pub locator: String,
    pub incarnation: String,
    pub host: Option<String>,
    pub boot: Option<String>,
    pub namespace: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "reference", rename_all = "snake_case", deny_unknown_fields)]
pub enum EntityRef {
    Unknown { reason: String },
    Key { key: EntityKey },
    Symbolic { scope: String, locator: String },
}
impl EntityRef {
    pub fn locator(&self) -> &str {
        match self {
            Self::Unknown { .. } => "",
            Self::Key { key } => &key.locator,
            Self::Symbolic { locator, .. } => locator,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Validity {
    pub start_us: u64,
    pub end_us: u64,
    pub clock: String,
    pub uncertainty_us: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityNode {
    pub id: String,
    pub key: EntityKey,
    pub validity: Option<Validity>,
    pub evidence: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relationship {
    pub id: String,
    pub from: EntityKey,
    pub to: Vec<EntityRef>,
    pub kind: String,
    pub validity: Option<Validity>,
    pub complete: bool,
    pub evidence: Vec<String>,
    pub denied: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "record_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum MetadataRecord {
    Node { node: EntityNode },
    Relation { relation: Relationship },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum TimeSelector {
    Unspecified,
    At {
        time_us: u64,
        clock: String,
        uncertainty_us: u64,
    },
    During {
        start_us: u64,
        end_us: u64,
        clock: String,
        uncertainty_us: u64,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssociationQuery {
    pub id: String,
    pub from: EntityKey,
    pub relation: String,
    pub selector: TimeSelector,
    pub is_set: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssociationResult {
    pub from: EntityKey,
    pub state: QualityState,
    pub candidates: Vec<EntityRef>,
    pub evidence: Vec<String>,
    pub temporal_status: String,
    pub reason_code: String,
    pub complete: bool,
    pub source_visibility: Vec<String>,
    pub physical_identity_status: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnrichmentInputMeasurement {
    pub id: String,
    pub native: Value,
    pub emitter: EntityRef,
    pub subject: Option<EntityRef>,
    pub value: Value,
    pub unit: String,
    pub scope: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnrichedObservation {
    pub measurement: EnrichmentInputMeasurement,
    pub relationships: BTreeMap<String, AssociationResult>,
}
#[derive(Debug, Clone)]
pub struct IdentityGraph {
    nodes: BTreeMap<EntityKey, EntityNode>,
    relations: BTreeMap<String, Relationship>,
    records: Vec<MetadataRecord>,
}
fn invalid(reason: &str) -> PrototypeError {
    PrototypeError::new("identity_invalid", reason)
}
fn valid_key(key: &EntityKey) -> bool {
    !key.scope.is_empty()
        && !key.kind.is_empty()
        && !key.locator.is_empty()
        && !key.incarnation.is_empty()
}
fn valid_time(time: &Option<Validity>) -> bool {
    time.as_ref()
        .is_none_or(|t| t.start_us < t.end_us && !t.clock.is_empty())
}
fn valid_evidence(ids: &[String]) -> bool {
    !ids.is_empty() && ids.iter().all(|s| !s.is_empty())
}
impl IdentityGraph {
    pub fn from_records(records: &[MetadataRecord]) -> Result<Self> {
        if records.len() > 128 {
            return Err(invalid("metadata record limit"));
        }
        let mut nodes = BTreeMap::new();
        let mut relations = BTreeMap::new();
        let mut seen: BTreeMap<String, MetadataRecord> = BTreeMap::new();
        for record in records {
            let id = match record {
                MetadataRecord::Node { node } => &node.id,
                MetadataRecord::Relation { relation } => &relation.id,
            };
            if id.is_empty() {
                return Err(invalid("empty evidence id"));
            }
            if let Some(old) = seen.get(id) {
                if old != record {
                    return Err(PrototypeError::new(
                        "identity_conflict",
                        "same evidence id with different content",
                    ));
                }
                continue;
            }
            match record {
                MetadataRecord::Node { node } => {
                    if !valid_key(&node.key)
                        || !valid_time(&node.validity)
                        || !valid_evidence(&node.evidence)
                    {
                        return Err(invalid("invalid node identity/time/evidence"));
                    }
                    if nodes.insert(node.key.clone(), node.clone()).is_some() {
                        return Err(PrototypeError::new(
                            "identity_conflict",
                            "multiple node definitions for one incarnation",
                        ));
                    }
                }
                MetadataRecord::Relation { relation } => {
                    if !valid_key(&relation.from)
                        || relation.kind.is_empty()
                        || relation.to.len() > 32
                        || !valid_time(&relation.validity)
                        || !valid_evidence(&relation.evidence)
                    {
                        return Err(invalid("invalid relationship"));
                    }
                    if relation.denied && !relation.to.is_empty() {
                        return Err(invalid("denied relationship cannot assert targets"));
                    }
                    if relation.to.iter().any(|r| match r {
                        EntityRef::Unknown { .. } => true,
                        EntityRef::Key { key } => {
                            !valid_key(key) || key.scope != relation.from.scope
                        }
                        EntityRef::Symbolic { scope, locator } => {
                            scope != &relation.from.scope || locator.is_empty()
                        }
                    }) {
                        return Err(invalid("invalid/cross-scope target"));
                    }
                    relations.insert(id.clone(), relation.clone());
                }
            }
            seen.insert(id.clone(), record.clone());
        }
        if nodes.len() > 32 || relations.len() > 32 {
            return Err(invalid("entity/relationship limit"));
        }
        for r in relations.values() {
            if !nodes.contains_key(&r.from)
                || r.to
                    .iter()
                    .any(|t| matches!(t,EntityRef::Key{key} if !nodes.contains_key(key)))
            {
                return Err(invalid("dangling entity reference"));
            }
        }
        Ok(Self {
            nodes,
            relations,
            records: seen.into_values().collect(),
        })
    }
    pub fn with_records(&self, additional: &[MetadataRecord]) -> Result<Self> {
        let mut records = self.records.clone();
        records.extend_from_slice(additional);
        Self::from_records(&records)
    }
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
}
fn covers(validity: &Option<Validity>, query: &TimeSelector) -> bool {
    let Some(v) = validity else {
        return false;
    };
    if v.uncertainty_us != 0 {
        return false;
    }
    match query {
        TimeSelector::Unspecified => false,
        TimeSelector::At {
            time_us,
            clock,
            uncertainty_us,
        } => {
            *uncertainty_us == 0
                && clock == &v.clock
                && v.start_us <= *time_us
                && *time_us < v.end_us
        }
        TimeSelector::During {
            start_us,
            end_us,
            clock,
            uncertainty_us,
        } => {
            *uncertainty_us == 0
                && start_us < end_us
                && clock == &v.clock
                && v.start_us <= *start_us
                && *end_us <= v.end_us
        }
    }
}
pub fn resolve_association(graph: &IdentityGraph, query: &AssociationQuery) -> AssociationResult {
    let mut result = AssociationResult {
        from: query.from.clone(),
        state: QualityState::Unknown,
        candidates: Vec::new(),
        evidence: Vec::new(),
        temporal_status: "unresolved".into(),
        reason_code: "no_matching_evidence".into(),
        complete: false,
        source_visibility: Vec::new(),
        physical_identity_status: "unverified_synthetic".into(),
    };
    if !valid_key(&query.from) {
        result.reason_code = "incomplete_identity".into();
        return result;
    }
    let Some(node) = graph.nodes.get(&query.from) else {
        return result;
    };
    let timed = !matches!(query.selector, TimeSelector::Unspecified);
    if timed && !covers(&node.validity, &query.selector) {
        result.reason_code = "entity_time_or_clock_unresolved".into();
        return result;
    }
    let mut candidates = BTreeSet::new();
    let mut evidence = BTreeSet::new();
    let mut complete = true;
    let mut usable = false;
    for r in graph
        .relations
        .values()
        .filter(|r| r.from == query.from && r.kind == query.relation)
    {
        if r.denied {
            result.source_visibility.push("permission_denied".into());
            continue;
        }
        if timed&&(!covers(&r.validity,&query.selector)||r.to.iter().any(|t|matches!(t,EntityRef::Key{key} if !covers(&graph.nodes[key].validity,&query.selector)))){continue;}
        usable = true;
        complete &= r.complete;
        candidates.extend(r.to.iter().cloned());
        evidence.extend(r.evidence.iter().cloned());
        evidence.insert(r.id.clone());
    }
    result.source_visibility.sort();
    result.source_visibility.dedup();
    if !usable {
        if !result.source_visibility.is_empty() {
            result.reason_code = "visibility_denied".into();
        } else if timed {
            result.reason_code = "relationship_time_or_clock_unresolved".into();
        }
        return result;
    }
    evidence.extend(node.evidence.iter().cloned());
    evidence.insert(node.id.clone());
    result.evidence = evidence.into_iter().collect();
    result.candidates = candidates.into_iter().collect();
    result.complete = complete;
    result.temporal_status = if timed {
        "validated_fixture_interval"
    } else {
        "unspecified_input"
    }
    .into();
    result.state = if !query.is_set && result.candidates.len() > 1 {
        QualityState::Ambiguous
    } else if !complete {
        QualityState::Partial
    } else if !query.is_set && result.candidates.is_empty() {
        QualityState::Unknown
    } else if timed {
        QualityState::Resolved
    } else {
        QualityState::ReportedOnly
    };
    result.reason_code = match result.state {
        QualityState::Ambiguous => "multiple_candidates",
        QualityState::Partial => "incomplete_declared_set",
        QualityState::Resolved => "explicit_time_valid_evidence",
        QualityState::ReportedOnly => "declared_without_query_time",
        _ => "no_asserted_target",
    }
    .into();
    result
}
pub fn associate_observation(
    observation: &EnrichmentInputMeasurement,
    graph: &IdentityGraph,
    query: &AssociationQuery,
) -> Result<EnrichedObservation> {
    if observation.id.is_empty() {
        return Err(invalid("measurement id required"));
    }
    if ![Some(&observation.emitter), observation.subject.as_ref()]
        .into_iter()
        .flatten()
        .any(|r| matches!(r,EntityRef::Key{key} if key==&query.from))
    {
        return Err(PrototypeError::new(
            "observation_query_mismatch",
            "query requires an explicitly keyed emitter or subject",
        ));
    }
    Ok(EnrichedObservation {
        measurement: observation.clone(),
        relationships: BTreeMap::from([(
            query.relation.clone(),
            resolve_association(graph, query),
        )]),
    })
}
