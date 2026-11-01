use crate::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct TraceabilitySummary {
    pub metrics: usize,
    pub associations: usize,
    pub experiments: usize,
    pub selected_metrics: usize,
    pub selected_associations: usize,
}

fn nonempty(v: &Value, key: &str, file: &str) -> ValidationResult<()> {
    require(
        !string(v, key)?.trim().is_empty(),
        file,
        &format!("empty {key}"),
    )
}
fn value_set(items: &[Value], key: &str) -> ValidationResult<BTreeMap<String, Value>> {
    ids(items, key)?;
    items
        .iter()
        .map(|v| Ok((string(v, key)?.into(), v.clone())))
        .collect()
}
fn verify_refs(
    v: &Value,
    sources: &BTreeSet<String>,
    locals: &BTreeSet<String>,
    file: &str,
) -> ValidationResult<()> {
    require(
        strings(&v["pinned_upstream_ids"])?.is_subset(sources),
        file,
        "unknown pinned source ID",
    )?;
    let local = strings(&v["frozen_local_ids"])?;
    require(
        !local.is_empty() && local.is_subset(locals),
        file,
        "unknown or missing local baseline reference",
    )?;
    require(
        v["runtime_evidence"].as_array().is_some_and(Vec::is_empty),
        file,
        "runtime evidence must remain empty in Milestone 1",
    )
}

pub fn check_traceability(root: &Path) -> ValidationResult<TraceabilitySummary> {
    check_evidence(root)?;
    let b = load_baselines(root)?;
    let rf = "docs/prototype-experiments.json";
    let sf = "docs/prototype-status.json";
    require(
        !text(root, "docs/prototype-scope.md")?.trim().is_empty(),
        "docs/prototype-scope.md",
        "scope is empty",
    )?;
    let registry = json(root, rf)?;
    let status = json(root, sf)?;
    for (file, v) in [(rf, &registry), (sf, &status)] {
        require(
            v["schema_version"] == 1
                && v["evidence_bundle"] == "aether-source-evidence-v1"
                && v["environment_id"] == "aether-validation-synthetic-local-v1",
            file,
            "wrong schema, bundle or environment",
        )?;
    }
    require(
        status["experiment_registry"] == rf
            && status["evidence_manifest_sha256"]
                == sha256(&read(root, "evidence/manifest-v1.json")?),
        sf,
        "wrong registry or manifest hash",
    )?;
    let catalog = value_set(array(&b.catalog, "metrics")?, "id")?;
    let mapping = value_set(array(&b.coverage, "metrics")?, "id")?;
    let assoc_base = value_set(&b.associations, "id")?;
    let metrics = value_set(array(&status, "metrics")?, "id")?;
    let associations = value_set(array(&status, "associations")?, "id")?;
    for id in catalog.keys() {
        require(
            metrics.contains_key(id),
            sf,
            &format!("missing metric {id}"),
        )?;
    }
    require(
        metrics.len() == 250 && metrics.keys().eq(catalog.keys()),
        sf,
        "unexpected metric IDs",
    )?;
    require(
        associations.len() == 58 && associations.keys().eq(assoc_base.keys()),
        sf,
        "association IDs must match 58 baseline definitions",
    )?;
    let experiments = value_set(array(&registry, "experiments")?, "id")?;
    let expected: BTreeSet<String> = (1..=12).map(|i| format!("PX{i:02}")).collect();
    require(
        experiments.keys().cloned().collect::<BTreeSet<_>>() == expected,
        rf,
        "expected exactly PX01 through PX12",
    )?;
    let source_ids = ids(array(&b.manifest, "source_inputs")?, "id")?;
    let local_ids = ids(array(&b.manifest, "local_inputs")?, "id")?;
    let profiles: BTreeSet<String> = (1..=10).map(|i| format!("C{i:02}")).collect();
    let all_metric_ids: BTreeSet<_> = catalog.keys().cloned().collect();
    let all_assoc_ids: BTreeSet<_> = assoc_base.keys().cloned().collect();
    let mut metric_links: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut assoc_links: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (id, e) in &experiments {
        let context = format!("{rf} [{id}]");
        for key in [
            "title",
            "purpose",
            "input_specification",
            "positive_expected_result",
            "negative_expected_result",
            "limits_of_claim",
            "planned_fixture_id",
        ] {
            nonempty(e, key, &context)?;
        }
        require(
            e["synthetic"] == true
                && e["execution_status"] == "not_run"
                && e["actual_results"].as_array().is_some_and(Vec::is_empty),
            &context,
            "synthetic experiment execution must remain not_run with no actual results",
        )?;
        let milestones = array(e, "execution_milestones")?;
        require(
            !milestones.is_empty()
                && milestones
                    .iter()
                    .all(|v| v.as_u64().is_some_and(|n| (2..=7).contains(&n))),
            &context,
            "execution milestones must be later synthetic milestones 2-7",
        )?;
        let p = strings(&e["profile_ids"])?;
        require(p.is_subset(&profiles), &context, "unknown profile ID")?;
        let sources = strings(&e["pinned_source_ids"])?;
        require(
            sources.is_subset(&source_ids),
            &context,
            "unknown pinned source ID",
        )?;
        let locals = strings(&e["local_baseline_ids"])?;
        require(
            !locals.is_empty() && locals.is_subset(&local_ids),
            &context,
            "invalid local baseline IDs",
        )?;
        require(
            e["evidence_kind"]
                == if sources.is_empty() {
                    "baseline_only_synthetic"
                } else {
                    "pinned_software_reference_with_invented_values"
                },
            &context,
            "invented records must retain proper evidence kind",
        )?;
        let reqs = strings(&e["requirement_ids"])?;
        let assocs = strings(&e["association_ids"])?;
        require(
            reqs.is_subset(&all_metric_ids) && assocs.is_subset(&all_assoc_ids),
            &context,
            "unknown original requirement/association",
        )?;
        require(
            matches!(
                e["experiment_kind"].as_str(),
                Some("metric_behavior" | "pipeline_control" | "collection_control")
            ),
            &context,
            "unknown experiment kind",
        )?;
        if e["experiment_kind"] != "metric_behavior" {
            require(
                reqs.is_empty(),
                &context,
                "control experiment must not invent metric requirements",
            )?;
        }
        for mid in &reqs {
            require(
                strings(&mapping[mid]["metadata_dependencies"])?.is_subset(&assocs),
                &context,
                &format!("dropped metadata dependency for {mid}"),
            )?;
            metric_links
                .entry(mid.clone())
                .or_default()
                .insert(id.clone());
        }
        for aid in &assocs {
            assoc_links
                .entry(aid.clone())
                .or_default()
                .insert(id.clone());
        }
        let budget_path = format!("docs/prototype-budgets/{id}.json");
        require(
            e["budget_record"] == budget_path,
            &context,
            "broken budget link or unsafe path",
        )?;
        let budget = json(root, &budget_path)?;
        verify_budget(&budget, &b.budget, e, &budget_path, &profiles)?;
    }
    for (id, row) in &metrics {
        require(
            row["baseline_requirement"] == catalog[id],
            sf,
            &format!("changed baseline definition {id}"),
        )?;
        require(
            row["research_mapping"] == mapping[id],
            sf,
            &format!("changed research route/dependency {id}"),
        )?;
        verify_record(row, id, metric_links.get(id), &source_ids, &local_ids, sf)?;
        let expected_sources: BTreeSet<String> = metric_links
            .get(id)
            .into_iter()
            .flat_map(|links| links.iter())
            .flat_map(|eid| strings(&experiments[eid]["pinned_source_ids"]).unwrap_or_default())
            .collect();
        require(
            strings(&row["evidence_references"]["pinned_upstream_ids"])? == expected_sources,
            sf,
            &format!("source references disagree with selected experiments {id}"),
        )?;
    }
    for (id, row) in &associations {
        require(
            row["baseline_association"] == assoc_base[id],
            sf,
            &format!("changed association definition {id}"),
        )?;
        require(
            row["association_verification"] == "not_run",
            sf,
            &format!("association verification must remain not_run {id}"),
        )?;
        verify_record(row, id, assoc_links.get(id), &source_ids, &local_ids, sf)?;
        let expected_sources: BTreeSet<String> = assoc_links
            .get(id)
            .into_iter()
            .flat_map(|links| links.iter())
            .flat_map(|eid| strings(&experiments[eid]["pinned_source_ids"]).unwrap_or_default())
            .collect();
        require(
            strings(&row["evidence_references"]["pinned_upstream_ids"])? == expected_sources,
            sf,
            &format!("association source references disagree {id}"),
        )?;
    }
    // At least one selected requirement from every observability layer.
    for layer in 1..=4 {
        require(
            metrics.values().any(|r| {
                r["selected_for_fixture"] == true && r["baseline_requirement"]["layer"] == layer
            }),
            sf,
            "experiment selection must represent all four layers",
        )?;
    }
    Ok(TraceabilitySummary {
        metrics: metrics.len(),
        associations: associations.len(),
        experiments: experiments.len(),
        selected_metrics: metric_links.len(),
        selected_associations: assoc_links.len(),
    })
}
fn verify_record(
    row: &Value,
    id: &str,
    links: Option<&BTreeSet<String>>,
    sources: &BTreeSet<String>,
    locals: &BTreeSet<String>,
    file: &str,
) -> ValidationResult<()> {
    let expected = links.cloned().unwrap_or_default();
    let selected = !expected.is_empty();
    require(
        strings(&row["experiment_ids"])? == expected && row["selected_for_fixture"] == selected,
        file,
        &format!("selection links disagree {id}"),
    )?;
    require(
        row["prototype_state"]
            == if selected {
                "fixture_planned"
            } else {
                "not_prototyped"
            },
        file,
        &format!("planning state inconsistent {id}"),
    )?;
    for key in [
        "synthetic_execution_status",
        "deployed_schema_verification",
        "capability_verification",
    ] {
        require(
            row[key] == "not_run",
            file,
            &format!("{key} must remain not_run {id}"),
        )?;
    }
    require(
        array(row, "remaining_unproven")?
            .iter()
            .any(|v| v.as_str().is_some_and(|s| !s.trim().is_empty())),
        file,
        &format!("remaining limits missing {id}"),
    )?;
    verify_refs(&row["evidence_references"], sources, locals, file)
}
fn verify_budget(
    b: &Value,
    template: &Value,
    e: &Value,
    file: &str,
    profiles: &BTreeSet<String>,
) -> ValidationResult<()> {
    for key in template
        .as_object()
        .ok_or_else(|| error(file, "invalid template"))?
        .keys()
    {
        require(
            b.get(key).is_some(),
            file,
            &format!("template field missing {key}"),
        )?;
    }
    require(
        b["experiment_id"] == e["id"]
            && b["proposal_status"] == "proposed_not_agreed"
            && b["evidence_scope"] == "synthetic_fixture"
            && b["environment_id"] == "aether-validation-synthetic-local-v1"
            && b["evidence_bundle"] == "aether-source-evidence-v1",
        file,
        "budget context or proposal mismatch",
    )?;
    for (key, expected) in [
        ("id", "id"),
        ("requirement_ids", "requirement_ids"),
        ("association_ids", "association_ids"),
        ("source_profiles", "profile_ids"),
    ] {
        require(
            b["run"][key] == e[expected],
            file,
            &format!("budget run links disagree {key}"),
        )?;
    }
    require(
        b["run"]["reference_truth"]["positive_expected_result"] == e["positive_expected_result"]
            && b["run"]["reference_truth"]["negative_expected_result"]
                == e["negative_expected_result"],
        file,
        "budget expected answers disagree",
    )?;
    require(
        b["run"]["workload_shape"] == e["input_specification"],
        file,
        "budget input spec disagrees",
    )?;
    let ps = array(b, "profiles")?;
    require(ids(ps, "id")? == *profiles, file, "profile set mismatch")?;
    let selected = strings(&e["profile_ids"])?;
    for p in ps {
        require(
            p["selected_for_this_run"] == selected.contains(string(p, "id")?)
                && p["implementation_status"] == "not_implemented"
                && p["synthetic_verification"] == "not_run"
                && p["real_verification"] == "not_run",
            file,
            "profile selection or unverified state changed",
        )?;
    }
    for key in template["readiness"]
        .as_object()
        .ok_or_else(|| error(file, "invalid readiness template"))?
        .keys()
    {
        require(
            b["readiness"][key] == false,
            file,
            "acceptance readiness contradicts pending requirements/agreements",
        )?;
    }
    require(
        b["readiness"]
            .as_object()
            .is_some_and(|o| o.values().all(|v| v == false)),
        file,
        "invalid readiness state",
    )?;
    require(
        b["agreement"] == template["agreement"],
        file,
        "agreement provenance must not be invented",
    )?;
    require(
        b["overall_result"] == "not_run",
        file,
        "budget cannot claim an executed result",
    )?;
    for (key, value) in template["access"]
        .as_object()
        .ok_or_else(|| error(file, "invalid access template"))?
    {
        if key == "sensitive_label_filter" {
            nonempty(&b["access"], key, file)?;
        } else {
            require(
                b["access"][key] == *value,
                file,
                &format!("access must remain ungranted {key}"),
            )?;
        }
    }
    let criteria = array(b, "acceptance_criteria")?;
    require(
        ids(criteria, "id")? == ids(array(template, "acceptance_criteria")?, "id")?,
        file,
        "criterion IDs changed",
    )?;
    for c in criteria {
        for key in ["limit", "agreed_by", "agreed_before_run_at"] {
            require(
                c.get(key).is_some_and(Value::is_null),
                file,
                "actual acceptance limits/agreements must remain unset",
            )?;
        }
        require(
            c["result"] == "not_run" && c["evidence"].as_array().is_some_and(Vec::is_empty),
            file,
            "criterion falsely claims execution evidence",
        )?;
        nonempty(c, "formula_and_population", file)?;
        nonempty(c, "reference_and_uncertainty", file)?;
        require(
            c["required"].is_boolean(),
            file,
            "criterion applicability/required decision missing",
        )?;
    }
    let limits = b["finite_safety_limits"]
        .as_object()
        .ok_or_else(|| error(file, "missing safety envelope"))?;
    for key in template["finite_safety_limits"]
        .as_object()
        .ok_or_else(|| error(file, "invalid safety template"))?
        .keys()
    {
        require(
            limits.get(key).is_some_and(|v| !v.is_null()),
            file,
            &format!("finite proposed safety limit missing {key}"),
        )?;
    }
    for key in [
        "max_targets",
        "max_response_bytes",
        "max_series",
        "max_label_bytes",
        "max_concurrency",
        "max_requests_per_second",
        "max_memory_bytes",
        "max_queue_bytes",
        "max_disk_bytes",
        "max_network_bytes_per_second",
    ] {
        require(
            b["finite_safety_limits"][key]
                .as_u64()
                .is_some_and(|n| n > 0),
            file,
            &format!("invalid finite positive limit {key}"),
        )?;
    }
    for (key, unit) in [
        ("request_timeout", "milliseconds"),
        ("poll_interval", "milliseconds"),
        ("max_cpu", "core_equivalents"),
        ("max_retention", "seconds"),
        ("max_run_duration", "seconds"),
    ] {
        require(
            b["finite_safety_limits"][key]["value"]
                .as_f64()
                .is_some_and(|n| n.is_finite() && n > 0.0)
                && b["finite_safety_limits"][key]["unit"] == unit,
            file,
            &format!("invalid finite positive limit {key}"),
        )?;
    }
    require(
        b["finite_safety_limits"]["retry_and_backoff_limits"]["max_retries"]
            .as_u64()
            .is_some()
            && b["finite_safety_limits"]["retry_and_backoff_limits"]["max_backoff_ms"]
                .as_u64()
                .is_some_and(|n| n > 0),
        file,
        "retry/backoff must be finite",
    )?;
    require(
        b["finite_safety_limits"]["on_exceed_action"] == "stop_case_and_report",
        file,
        "safety action must stop/report",
    )?;
    if e["id"] == "PX12" {
        require(
            b["finite_safety_limits"]["queue_record_limit_for_deliberate_test"] == 2,
            file,
            "PX12 must retain declared two-record queue",
        )?;
    }
    Ok(())
}
