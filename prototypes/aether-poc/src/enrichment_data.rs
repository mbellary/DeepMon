//! Independently authored invented inputs and literal expectations; no resolver calls.
use crate::{
    PrototypeError, Result, enrichment_fixture::*, fixture::Artifact, identity::*,
    observation::QualityState,
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path};
fn key(locator: &str, incarnation: &str) -> EntityKey {
    EntityKey {
        scope: "boundary-fixture".into(),
        kind: "process".into(),
        locator: locator.into(),
        incarnation: incarnation.into(),
        host: None,
        boot: None,
        namespace: None,
    }
}
fn validity(start: u64, end: u64) -> Option<Validity> {
    Some(Validity {
        start_us: start,
        end_us: end,
        clock: "fixture".into(),
        uncertainty_us: 0,
    })
}
fn node(id: &str, key: &EntityKey) -> MetadataRecord {
    MetadataRecord::Node {
        node: EntityNode {
            id: id.into(),
            key: key.clone(),
            validity: validity(0, 20),
            evidence: vec![id.into()],
        },
    }
}
fn relation(
    id: &str,
    from: &EntityKey,
    kind: &str,
    to: &[&str],
    time: Option<Validity>,
) -> MetadataRecord {
    MetadataRecord::Relation {
        relation: Relationship {
            id: id.into(),
            from: from.clone(),
            kind: kind.into(),
            to: to
                .iter()
                .map(|v| EntityRef::Symbolic {
                    scope: from.scope.clone(),
                    locator: (*v).into(),
                })
                .collect(),
            validity: time,
            complete: true,
            evidence: vec![id.into()],
            denied: false,
        },
    }
}
fn query(
    id: &str,
    from: &EntityKey,
    relation: &str,
    time: Option<u64>,
    is_set: bool,
) -> AssociationQuery {
    AssociationQuery {
        id: id.into(),
        from: from.clone(),
        relation: relation.into(),
        selector: time.map_or(TimeSelector::Unspecified, |time_us| TimeSelector::At {
            time_us,
            clock: "fixture".into(),
            uncertainty_us: 0,
        }),
        is_set,
    }
}
fn expected(
    state: QualityState,
    candidates: &[&str],
    temporal: Option<&str>,
) -> ExpectedAssociation {
    ExpectedAssociation {
        state,
        candidates: candidates.iter().map(|v| (*v).into()).collect(),
        temporal_status: temporal.map(str::to_string),
    }
}
fn measurement(id: &str, from: &EntityKey, value: Value) -> EnrichmentInputMeasurement {
    EnrichmentInputMeasurement {
        id: id.into(),
        native: json!({"family":"fixture_requests","type":"counter","labels":{"model":"same-model"}}),
        emitter: EntityRef::Key { key: from.clone() },
        subject: Some(EntityRef::Key { key: from.clone() }),
        value,
        unit: "requests".into(),
        scope: "replica".into(),
    }
}
fn authored() -> Vec<(BoundaryInput, BoundaryExpected)> {
    use QualityState::*;
    let mut cases = Vec::new();
    let blank = |id: &str| BoundaryInput {
        schema_version: 1,
        synthetic: true,
        case_id: id.into(),
        records: vec![],
        queries: vec![],
        measurements: vec![],
        measurement_queries: BTreeMap::new(),
        updates: vec![],
        after_queries: vec![],
    };
    let oracle = |id: &str| BoundaryExpected {
        schema_version: 1,
        synthetic: true,
        case_id: id.into(),
        queries: BTreeMap::new(),
        measurement_values: BTreeMap::new(),
        update_error: None,
        history_unchanged: true,
        permutation_invariant: true,
    };
    let old = key("PID17", "start-one");
    let new = key("PID17", "start-two");
    let mut input = blank("E5-01");
    let mut answer = oracle("E5-01");
    let mut old_node = node("old", &old);
    if let MetadataRecord::Node { node } = &mut old_node {
        node.validity = validity(0, 10);
    }
    let mut new_node = node("new", &new);
    if let MetadataRecord::Node { node } = &mut new_node {
        node.validity = validity(10, 20);
    }
    input.records = vec![
        old_node,
        new_node,
        relation(
            "old-membership",
            &old,
            "container",
            &["container-A"],
            validity(0, 10),
        ),
        relation(
            "new-membership",
            &new,
            "container",
            &["container-B"],
            validity(10, 20),
        ),
    ];
    for (id, from, time, state, candidates) in [
        ("old-at-5", &old, 5, Resolved, vec!["container-A"]),
        ("old-at-10", &old, 10, Unknown, vec![]),
        ("new-at-10", &new, 10, Resolved, vec!["container-B"]),
        ("new-at-15", &new, 15, Resolved, vec!["container-B"]),
        ("old-at-20", &old, 20, Unknown, vec![]),
        ("new-at-20", &new, 20, Unknown, vec![]),
    ] {
        input
            .queries
            .push(query(id, from, "container", Some(time), false));
        answer
            .queries
            .insert(id.into(), expected(state, &candidates, None));
    }
    let mut crossing = query("crossing", &old, "container", None, false);
    crossing.selector = TimeSelector::During {
        start_us: 5,
        end_us: 15,
        clock: "fixture".into(),
        uncertainty_us: 0,
    };
    input.queries.push(crossing);
    answer
        .queries
        .insert("crossing".into(), expected(Unknown, &[], None));
    cases.push((input, answer));
    let p = key("publisher", "one");
    let mut input = blank("E5-02");
    let mut answer = oracle("E5-02");
    input.records = vec![
        node("publisher", &p),
        relation("one", &p, "one", &["p1"], validity(0, 20)),
        relation("two", &p, "publisher", &["p1", "p2"], validity(0, 20)),
    ];
    for (id, kind, state, values) in [
        ("zero", "absent", Unknown, vec![]),
        ("one", "one", Resolved, vec!["p1"]),
        ("two", "publisher", Ambiguous, vec!["p1", "p2"]),
    ] {
        input.queries.push(query(id, &p, kind, Some(5), false));
        answer
            .queries
            .insert(id.into(), expected(state, &values, None));
    }
    cases.push((input, answer));
    let mut a = key("17", "start");
    a.host = Some("host-A".into());
    a.boot = Some("boot-A".into());
    a.namespace = Some("pidns-A".into());
    let mut b = a.clone();
    b.boot = Some("boot-B".into());
    b.namespace = Some("pidns-B".into());
    let mut input = blank("E5-03");
    let mut answer = oracle("E5-03");
    input.records = vec![
        node("A", &a),
        node("B", &b),
        relation(
            "A-owner",
            &a,
            "container",
            &["container-A"],
            validity(0, 20),
        ),
        relation(
            "B-owner",
            &b,
            "container",
            &["container-B"],
            validity(0, 20),
        ),
    ];
    for (id, k, values) in [
        ("boot-A", a.clone(), vec!["container-A"]),
        ("boot-B", b.clone(), vec!["container-B"]),
    ] {
        input
            .queries
            .push(query(id, &k, "container", Some(5), false));
        answer
            .queries
            .insert(id.into(), expected(Resolved, &values, None));
    }
    let mut missing = a.clone();
    missing.incarnation.clear();
    input.queries.push(query(
        "missing-incarnation",
        &missing,
        "container",
        Some(5),
        false,
    ));
    answer
        .queries
        .insert("missing-incarnation".into(), expected(Unknown, &[], None));
    let mut wrong = a;
    wrong.namespace = Some("unknown-pidns".into());
    input.queries.push(query(
        "wrong-namespace",
        &wrong,
        "container",
        Some(5),
        false,
    ));
    answer
        .queries
        .insert("wrong-namespace".into(), expected(Unknown, &[], None));
    cases.push((input, answer));
    let mut replica = key("replica-A", "generation-one");
    replica.kind = "replica".into();
    let mut other = replica.clone();
    other.locator = "replica-B".into();
    let mut input = blank("E5-04");
    let mut answer = oracle("E5-04");
    input.records = vec![
        node("replica-A", &replica),
        node("replica-B", &other),
        relation(
            "allocation",
            &replica,
            "allocated_resources",
            &["gpu-A", "gpu-B"],
            None,
        ),
    ];
    input.queries = vec![
        query("allocation", &replica, "allocated_resources", None, true),
        query(
            "participation",
            &replica,
            "observed_participants",
            None,
            true,
        ),
        query("other-replica", &other, "allocated_resources", None, true),
    ];
    answer.queries.insert(
        "allocation".into(),
        expected(ReportedOnly, &["gpu-A", "gpu-B"], Some("unspecified_input")),
    );
    answer
        .queries
        .insert("participation".into(), expected(Unknown, &[], None));
    answer
        .queries
        .insert("other-replica".into(), expected(Unknown, &[], None));
    let mut m = measurement("count-100", &replica, json!(100));
    m.native["declared_models"] = json!(["symbolic-A", "symbolic-B"]);
    input.measurements.push(m);
    input
        .measurement_queries
        .insert("count-100".into(), "allocation".into());
    answer
        .measurement_values
        .insert("count-100".into(), json!(100));
    cases.push((input, answer));
    let mut input = blank("E5-05");
    let mut answer = oracle("E5-05");
    input.records = vec![
        node("replica", &replica),
        relation(
            "initial-members",
            &replica,
            "allocated_resources",
            &["gpu-A"],
            validity(0, 10),
        ),
    ];
    input.queries = vec![query("old", &replica, "allocated_resources", Some(5), true)];
    input.updates = vec![relation(
        "new-members",
        &replica,
        "allocated_resources",
        &["gpu-B"],
        validity(10, 20),
    )];
    input.after_queries = vec![
        query("old-after", &replica, "allocated_resources", Some(5), true),
        query("new-after", &replica, "allocated_resources", Some(15), true),
    ];
    input
        .measurements
        .push(measurement("historic-count", &replica, json!(100)));
    input
        .measurement_queries
        .insert("historic-count".into(), "old".into());
    answer
        .measurement_values
        .insert("historic-count".into(), json!(100));
    for (id, values) in [
        ("old", vec!["gpu-A"]),
        ("old-after", vec!["gpu-A"]),
        ("new-after", vec!["gpu-B"]),
    ] {
        answer
            .queries
            .insert(id.into(), expected(Resolved, &values, None));
    }
    cases.push((input, answer));
    let mut nic = key("ifindex7", "link-one");
    nic.kind = "nic".into();
    nic.namespace = Some("netns-A".into());
    let mut nic_b = nic.clone();
    nic_b.namespace = Some("netns-B".into());
    let mut input = blank("E5-06");
    let mut answer = oracle("E5-06");
    input.records = vec![
        node("nic-A", &nic),
        node("nic-B", &nic_b),
        relation(
            "capability",
            &nic,
            "capabilities",
            &["ethernet", "rdma"],
            None,
        ),
        relation(
            "configuration",
            &nic,
            "configured_preference",
            &["roce"],
            None,
        ),
        relation("control", &nic, "control_protocol", &["tcp"], None),
    ];
    for (id, k, kind, state, values) in [
        (
            "capabilities",
            &nic,
            "capabilities",
            ReportedOnly,
            vec!["ethernet", "rdma"],
        ),
        (
            "configuration",
            &nic,
            "configured_preference",
            ReportedOnly,
            vec!["roce"],
        ),
        ("active", &nic, "active_transport", Unknown, vec![]),
        ("operation", &nic, "operation_to_nic", Unknown, vec![]),
        ("libraries", &nic, "loaded_cuda_nccl", Unknown, vec![]),
        ("other-netns", &nic_b, "capabilities", Unknown, vec![]),
    ] {
        input.queries.push(query(id, k, kind, None, true));
        answer
            .queries
            .insert(id.into(), expected(state, &values, None));
    }
    cases.push((input, answer));
    let process = key("PID17", "start-two");
    let mut input = blank("E5-07");
    let mut answer = oracle("E5-07");
    let n = node("known-process", &process);
    let mut denied = relation(
        "runtime-denied",
        &process,
        "container",
        &[],
        validity(0, 20),
    );
    if let MetadataRecord::Relation { relation } = &mut denied {
        relation.denied = true;
        relation.complete = false;
    }
    input.records = vec![n.clone(), n.clone(), denied];
    input.queries = vec![query("denied", &process, "container", Some(5), false)];
    answer
        .queries
        .insert("denied".into(), expected(Unknown, &[], None));
    let mut m = measurement("safe-measurement", &process, json!(1));
    m.native = json!({"fixture":"safe","secret":"SYNTHETIC_ONLY_SECRET_MARKER"});
    input.measurements.push(m);
    input
        .measurement_queries
        .insert("safe-measurement".into(), "denied".into());
    answer
        .measurement_values
        .insert("safe-measurement".into(), json!(1));
    let mut conflicting = n;
    if let MetadataRecord::Node { node } = &mut conflicting {
        node.validity = validity(0, 19);
    }
    input.updates = vec![conflicting];
    answer.update_error = Some("identity_conflict".into());
    cases.push((input, answer));
    cases
}
pub fn author_enrichment_fixtures(root: &Path) -> Result<usize> {
    let base = root.join("fixtures/enrichment-v1");
    let mut outputs = BTreeMap::new();
    let mut cases = Vec::new();
    for (input, expected) in authored() {
        let mut artifacts = Vec::new();
        for (role, value) in [
            ("input", serde_json::to_value(&input)),
            ("expected", serde_json::to_value(&expected)),
        ] {
            let value =
                value.map_err(|_| PrototypeError::new("authoring_error", "serialization"))?;
            let mut bytes = serde_json::to_vec_pretty(&value)
                .map_err(|_| PrototypeError::new("authoring_error", "serialization"))?;
            bytes.push(b'\n');
            let path = format!("{role}/{}.json", input.case_id);
            artifacts.push(Artifact {
                path: path.clone(),
                role: role.into(),
                media_type: "application/json".into(),
                bytes: bytes.len() as u64,
                sha256: aether_validation::sha256(&bytes),
            });
            outputs.insert(path, bytes);
        }
        cases.push(CaseIndex {
            case_id: input.case_id,
            input: artifacts.remove(0),
            expected: artifacts.remove(0),
        });
    }
    let manifest = ExtensionManifest {
        schema_version: 1,
        synthetic: true,
        bundle_id: "aether-enrichment-fixtures-v1".into(),
        original_manifest_sha256: aether_validation::sha256(&fs::read(
            root.join("fixtures/prototype/manifest.json"),
        )?),
        associations_sha256: aether_validation::sha256(&fs::read(
            root.join("metadata-enrichment-matrix.md"),
        )?),
        registry_sha256: aether_validation::sha256(&fs::read(
            root.join("docs/prototype-experiments.json"),
        )?),
        cases,
        limits_of_claim:
            "Invented local identity tests; no captured device/runtime data or budget acceptance."
                .into(),
    };
    let mut bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|_| PrototypeError::new("authoring_error", "serialization"))?;
    bytes.push(b'\n');
    outputs.insert("manifest.json".into(), bytes);
    outputs.insert("README.md".into(),b"# Enrichment boundary fixtures v1\n\nSeven explicitly invented cases. Independent expected answers are authored literals, never resolver output. Query times, identities, models and values are test choices. Original fixture/source/registry hashes are pinned in manifest.json. No real acquisition or production support is established.\n".to_vec());
    for (path, bytes) in &outputs {
        let dest = base.join(path);
        if dest.exists() && fs::read(dest)? != *bytes {
            return Err(PrototypeError::new(
                "authoring_conflict",
                "refusing differing fixture output",
            ));
        }
    }
    let mut written = 0;
    for (path, bytes) in outputs {
        let dest = base.join(path);
        if !dest.exists() {
            fs::create_dir_all(dest.parent().unwrap())?;
            use std::io::Write;
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dest)?;
            file.write_all(&bytes)?;
            written += 1;
        }
    }
    Ok(written)
}
