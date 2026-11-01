use crate::*;
use serde_json::json;
use std::collections::BTreeSet;
use std::io::Write;

struct Case {
    title: &'static str,
    metrics: &'static [&'static str],
    associations: &'static [&'static str],
    profiles: &'static [&'static str],
    sources: &'static [&'static str],
    milestones: &'static [u8],
    input: &'static str,
    positive: &'static str,
    negative: &'static str,
    limit: &'static str,
    kind: &'static str,
}
fn cases() -> Vec<Case> {
    vec![
        Case {
            title: "Native families and producer boundaries",
            metrics: &["L1-009", "L2-011", "L2-012", "L2-022", "L2-047", "L2-059"],
            associations: &[],
            profiles: &["C01", "C02", "C03", "C04", "C05", "C06"],
            sources: &[
                "E01", "E02", "E03", "E04", "E05", "E06", "E07", "E08", "E09", "E10",
            ],
            milestones: &[2, 3, 4],
            kind: "metric_behavior",
            input: "Six producer variants using each producer's own pinned families, HELP/TYPE and labels: vLLM iteration_tokens_total histogram, SGLang latency, Dynamo frontend/backend views, TGI batch size, Triton shared pinned pool and TensorRT block/iteration gauges.",
            positive: "Preserve native name, type, unit, labels and producer scope; a _total suffix does not override a histogram TYPE; Triton shared pool remains server-wide.",
            negative: "Malformed types fail explicitly; absent optional statistics remain unavailable, not zero; no snapshot becomes raw events.",
            limit: "Pinned names are documentation evidence, not running release compatibility; per-family mappings remain distinct.",
        },
        Case {
            title: "Counter rates, resets and unresolved arrivals",
            metrics: &["L1-001", "L1-002", "L1-005", "L1-006"],
            associations: &[],
            profiles: &["C04", "C05"],
            sources: &["E06", "E07", "E08"],
            milestones: &[2, 4],
            kind: "metric_behavior",
            input: "Synthetic Triton success counter 100 at virtual t=0 seconds, 150 at t=10; a reset variant; separately preserve raw TGI request_count.",
            positive: "Source-scoped success rate is (150-100)/10 = 5 requests/second; reset is identified without a negative rate.",
            negative: "Raw TGI counts do not establish accepted-inference requests; L1-001 and L1-005 normalization remains unresolved.",
            limit: "Invented counter values do not resolve actual API admission semantics or prove complete terminal populations.",
        },
        Case {
            title: "Insufficient aggregate evidence",
            metrics: &["L1-025", "L1-027"],
            associations: &["M16"],
            profiles: &["C01", "C02", "C05"],
            sources: &[],
            milestones: &[2, 4, 7],
            kind: "metric_behavior",
            input: "Aggregate timing histograms and separate success counts, with no joint per-request records or second moments.",
            positive: "Retain and report the available aggregates with their source boundaries.",
            negative: "Exact joint goodput and exact variance return insufficient-input; no invented request events or exact values are emitted.",
            limit: "This is a negative semantic experiment from the requirement baseline, not a newly supported per-request integration.",
        },
        Case {
            title: "Framework/backend duplication and remote subject",
            metrics: &["L1-002", "L1-009"],
            associations: &["M02", "M03", "M09", "M11", "M12", "M31"],
            profiles: &["C01", "C02", "C03", "C10"],
            sources: &["E02", "E03", "E04", "E05"],
            milestones: &[2, 4, 5],
            kind: "metric_behavior",
            input: "Frontend and backend observations referring to the same ten logical requests; frontend also reports a worker whose location evidence is missing.",
            positive: "An explicit source counting boundary reports ten logical requests, not twenty; each raw series retains its producer.",
            negative: "Worker host is unknown, not the frontend host; timing distributions are not merged as interchangeable measurements.",
            limit: "Logical overlap is supplied by invented linkage and is not inferred as a general live-engine capability.",
        },
        Case {
            title: "Replica GPU set and memory occupancy",
            metrics: &["L1-002", "L4-007", "L4-008", "L4-009", "L4-010"],
            associations: &["M09", "M32", "M33", "M37", "M38", "M40"],
            profiles: &["C08", "C10"],
            sources: &[],
            milestones: &[2, 4, 5],
            kind: "metric_behavior",
            input: "Replica-wide count 100; two symbolic GPU models; used/total memory 10/20 and 20/40 in matching invented units.",
            positive: "Replica count remains 100, not 200; each device has occupancy 0.5; device membership is a set.",
            negative: "Missing total prevents occupancy; incomplete membership remains partial rather than a single verified GPU-model label.",
            limit: "Symbolic device inputs are baseline-only synthetic records, not captures or certified hardware schemas.",
        },
        Case {
            title: "Process and cgroup incarnation changes",
            metrics: &["L4-048", "L4-054"],
            associations: &["M21", "M22", "M23", "M24", "M25", "M26", "M30", "M58"],
            profiles: &["C07", "C10"],
            sources: &[],
            milestones: &[2, 5],
            kind: "metric_behavior",
            input: "Same invented PID with a new process-start identity; changed cgroup membership; denied runtime metadata.",
            positive: "Historical observations retain the old incarnation and membership; later observations use new evidence with validity windows.",
            negative: "No guessed container on denied metadata, and no historical relabeling through PID reuse.",
            limit: "Tests identity logic, not actual procfs/runtime discovery or backward compatibility.",
        },
        Case {
            title: "NIC capability is not observed transport",
            metrics: &["L4-071"],
            associations: &[
                "M43", "M44", "M45", "M46", "M47", "M48", "M49", "M50", "M51", "M52", "M53", "M55",
                "M56",
            ],
            profiles: &["C09", "C10"],
            sources: &[],
            milestones: &[2, 5],
            kind: "metric_behavior",
            input: "Symbolic multi-transport adapter, source-scoped interface byte counters and a configured preference; no payload-operation path record.",
            positive: "Preserve capability, configured preference and byte counters independently.",
            negative: "Active transport and operation-to-NIC attribution remain unknown; no RoCE/InfiniBand/TCP conclusion from one control socket.",
            limit: "No real NIC model, fabric mode or hardware counter support is selected or proven.",
        },
        Case {
            title: "Distributed rank skew and clock uncertainty",
            metrics: &["L3-001", "L3-007", "L3-008"],
            associations: &["M13", "M14", "M15", "M16"],
            profiles: &["C10"],
            sources: &[],
            milestones: &[2, 7],
            kind: "metric_behavior",
            input: "Complete two-rank group arriving at 100 and 130 virtual microseconds in one declared clock domain; variants with missing member, mismatched group incarnation or unknown offset.",
            positive: "Arrival skew is 30 virtual microseconds with complete membership and a stated clock/uncertainty model.",
            negative: "Incomplete membership or uncertain cross-clock offsets prevent exact whole-group skew; bounded estimates must retain uncertainty.",
            limit: "Tests arithmetic on invented events, not physical timing accuracy or an implemented collective collector.",
        },
        Case {
            title: "Stream gap versus whole-device idle",
            metrics: &["L4-040"],
            associations: &[],
            profiles: &["C10"],
            sources: &[],
            milestones: &[2, 7],
            kind: "metric_behavior",
            input: "In virtual microseconds, stream A covers [0,10] and [20,30]; B covers [10,20]; observation window [0,30].",
            positive: "Stream A has a ten-microsecond gap while complete device coverage has zero idle time in the window.",
            negative: "Missing stream coverage prevents a whole-device idle claim.",
            limit: "Invented device events do not qualify a real profiling or microsecond acquisition path.",
        },
        Case {
            title: "Storage admission, commit and replay",
            metrics: &[],
            associations: &[],
            profiles: &[],
            sources: &[],
            milestones: &[2, 6],
            kind: "pipeline_control",
            input: "Two stable record IDs; interruptions before and after the declared simulated durability/commit boundary; pre-persistence volatile-loss variant.",
            positive: "Within the protected boundary, recovery yields exactly two unique committed records; query excludes uncommitted files.",
            negative: "Pre-persistence loss is explicitly accounted for; admitted is not durable and retries cannot multiply logical records.",
            limit: "Storage mechanics and durability are future local experiments; no production zero-loss or object-store guarantee.",
        },
        Case {
            title: "Visibility, privacy and action policy",
            metrics: &[],
            associations: &["M58"],
            profiles: &[
                "C01", "C02", "C03", "C04", "C05", "C06", "C07", "C08", "C09", "C10",
            ],
            sources: &[],
            milestones: &[2, 3, 4, 5],
            kind: "collection_control",
            input: "Invented denied source, disabled family, missing identity, sensitive labels and required-null acceptance limit; attempts at forbidden activation/escalation/workload mutation.",
            positive: "Keep denied, disabled and unknown states explicit; sanitize retained data; no secret values or private payloads.",
            negative: "Forbidden actions are rejected; required-null acceptance prevents readiness/pass rather than enabling telemetry automatically.",
            limit: "Planned policy assertions grant no real access and do not constitute an executed collector policy test.",
        },
        Case {
            title: "Latest-gauge freshness and bounded admission",
            metrics: &["L2-059"],
            associations: &["M58"],
            profiles: &["C06"],
            sources: &["E09"],
            milestones: &[2, 4, 6],
            kind: "metric_behavior",
            input: "Two equal latest iteration-gauge snapshots; separate pipeline assertion with queue capacity two records and three offered stable IDs under reject-new policy.",
            positive: "State source freshness uncertainty; the pipeline assertion admits two and explicitly rejects one.",
            negative: "No inference of absent intervening updates, complete iteration coverage or lossless delivery of all three records.",
            limit: "Queue count and virtual times are fixture values, not throughput, physical resolution or production loss promises.",
        },
    ]
}

fn write_outputs(root: &Path, outputs: Vec<(String, Vec<u8>)>) -> ValidationResult<usize> {
    let base = root.canonicalize().map_err(|e| error("root", e))?;
    // Inspect all destinations before any write: preserve existing/manual content.
    for (relative, bytes) in &outputs {
        let mut parent = base.clone();
        for component in Path::new(relative)
            .parent()
            .ok_or_else(|| error(relative, "no parent"))?
            .components()
        {
            require(
                matches!(component, Component::Normal(_)),
                relative,
                "unsafe output path",
            )?;
            parent.push(component);
            if parent.exists() || parent.is_symlink() {
                let actual = parent.canonicalize().map_err(|e| error(relative, e))?;
                require(
                    actual.starts_with(&base),
                    relative,
                    "output parent escapes root",
                )?;
            }
        }
        let path = base.join(relative);
        if path.exists() || path.is_symlink() {
            let actual = safe_path(root, relative)?;
            require(
                fs::read(actual).map_err(|e| error(relative, e))? == *bytes,
                relative,
                "refusing to overwrite existing/manual content",
            )?;
        }
    }
    let mut created = 0;
    for (relative, bytes) in outputs {
        let path = base.join(&relative);
        if path.exists() {
            continue;
        }
        fs::create_dir_all(path.parent().ok_or_else(|| error(&relative, "no parent"))?)
            .map_err(|e| error(&relative, e))?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| error(&relative, e))?;
        file.write_all(&bytes).map_err(|e| error(&relative, e))?;
        created += 1;
    }
    Ok(created)
}
fn encoded(v: &Value) -> ValidationResult<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(v).map_err(|e| error("serialization", e))?;
    bytes.push(b'\n');
    Ok(bytes)
}
pub fn generate_records(root: &Path) -> ValidationResult<usize> {
    check_evidence(root)?;
    let b = load_baselines(root)?;
    let mappings = array(&b.coverage, "metrics")?;
    let manifest_hash = sha256(&read(root, "evidence/manifest-v1.json")?);
    let mut experiments = Vec::new();
    let mut outputs = Vec::new();
    for (index, case) in cases().into_iter().enumerate() {
        let id = format!("PX{:02}", index + 1);
        let mut assocs: BTreeSet<String> =
            case.associations.iter().map(|s| s.to_string()).collect();
        for mid in case.metrics {
            let mapping = mappings
                .iter()
                .find(|m| m["id"] == *mid)
                .ok_or_else(|| error(mid, "unknown experiment metric"))?;
            assocs.extend(strings(&mapping["metadata_dependencies"])?);
        }
        let path = format!("docs/prototype-budgets/{id}.json");
        let experiment = json!({"id":id,"title":case.title,"purpose":format!("Specify and later evaluate: {}",case.title),"synthetic":true,"experiment_kind":case.kind,"execution_milestones":case.milestones,"profile_ids":case.profiles,"requirement_ids":case.metrics,"association_ids":assocs,"pinned_source_ids":case.sources,"local_baseline_ids":["B01","B02","B03","B05","B06"],"evidence_kind":if case.sources.is_empty(){"baseline_only_synthetic"}else{"pinned_software_reference_with_invented_values"},"input_specification":case.input,"positive_expected_result":case.positive,"negative_expected_result":case.negative,"limits_of_claim":case.limit,"budget_record":path,"execution_status":"not_run","actual_results":[],"planned_fixture_id":format!("synthetic-{id}-v1")});
        let mut budget = b.budget.clone();
        budget["experiment_id"] = json!(id);
        budget["proposal_status"] = json!("proposed_not_agreed");
        budget["evidence_scope"] = json!("synthetic_fixture");
        budget["environment_id"] = json!("aether-validation-synthetic-local-v1");
        budget["evidence_bundle"] = b.manifest["bundle_id"].clone();
        for profile in budget["profiles"]
            .as_array_mut()
            .ok_or_else(|| error(&path, "missing profiles"))?
        {
            profile["selected_for_this_run"] = json!(
                case.profiles
                    .contains(&profile["id"].as_str().unwrap_or(""))
            );
        }
        budget["run"]["id"] = json!(id);
        budget["run"]["manifest_revision"] = json!(1);
        budget["run"]["source_profiles"] = experiment["profile_ids"].clone();
        budget["run"]["requirement_ids"] = experiment["requirement_ids"].clone();
        budget["run"]["association_ids"] = experiment["association_ids"].clone();
        budget["run"]["source_or_fixture_versions"] = json!([{"bundle":"aether-source-evidence-v1","manifest_sha256":manifest_hash,"pinned_source_ids":case.sources,"planned_fixture_id":experiment["planned_fixture_id"]}]);
        budget["run"]["environment_description"] =
            json!("Local synthetic reference only; no real engine/hardware access.");
        budget["run"]["workload_shape"] = json!(case.input);
        budget["run"]["offered_metric_volume_and_cardinality"] = json!(
            "Up to 64 series per variant; at most 32 invented entities; actual fixture manifest pending."
        );
        budget["run"]["warmup"] =
            json!("No real workload; virtual warmup only if later specified.");
        budget["run"]["measurement_window"] =
            json!("Declared virtual case window; wall-clock scenario cap 60 seconds.");
        budget["run"]["repetitions"] = json!(1);
        budget["run"]["baseline_and_pairing_method"] = json!(
            "Compare against hand-checkable invented expected states, never a real inference baseline."
        );
        budget["run"]["fault_scenarios"] = json!([case.negative]);
        budget["run"]["reference_truth"] = json!({"positive_expected_result":case.positive,"negative_expected_result":case.negative});
        budget["run"]["durability_boundary"] = json!(if index == 9 {
            "Simulated persistence/commit boundary must be fixed by Milestone 6; no real durability claim."
        } else {
            "No persistence acceptance in this case unless separately declared."
        });
        budget["run"]["source_to_subject_identity_evidence"] = json!(
            "Invented explicit identities and validity windows; missing relations stay unknown."
        );
        budget["finite_safety_limits"] = json!({"max_targets":32,"max_invented_entities":32,"max_response_bytes":1048576,"max_series":64,"max_label_bytes":256,"request_timeout":{"value":1000,"unit":"milliseconds"},"max_concurrency":1,"poll_interval":{"value":100,"unit":"milliseconds","status":"proposed_only"},"max_requests_per_second":10,"retry_and_backoff_limits":{"max_retries":2,"max_backoff_ms":1000},"max_cpu":{"value":1,"unit":"core_equivalents"},"max_memory_bytes":268435456,"max_queue_bytes":67108864,"max_disk_bytes":134217728,"max_network_bytes_per_second":1048576,"max_retention":{"value":600,"unit":"seconds"},"max_run_duration":{"value":60,"unit":"seconds"},"on_exceed_action":"stop_case_and_report","queue_record_limit_for_deliberate_test":if index==11 {json!(2)}else{Value::Null}});
        budget["targets_status"] = json!("pending_fixture_creation_no_targets_authorized");
        budget["access"]["sensitive_label_filter"] = json!(
            "Synthetic allowlisted labels only; no secrets, prompts, responses, tensor/KV content or process environments."
        );
        let window = budget["run"]["measurement_window"].clone();
        for criterion in budget["acceptance_criteria"]
            .as_array_mut()
            .ok_or_else(|| error(&path, "missing criteria"))?
        {
            let cid = criterion["id"].as_str().unwrap_or("").to_owned();
            let correctness = matches!(
                cid.as_str(),
                "loss" | "duplication_replay" | "attribution_semantics"
            );
            criterion["required"] = json!(correctness);
            criterion["applicability"] = json!(if correctness {
                "synthetic_correctness_proposed"
            } else {
                "undecided"
            });
            criterion["formula_and_population"] = json!(format!(
                "{}; evaluate only the named case population and declared observation boundary.",
                criterion["unit"].as_str().unwrap_or("")
            ));
            criterion["percentile_and_aggregation_rule"] = json!(
                "Exact named outcomes for correctness; performance distributions not agreed."
            );
            criterion["window"] = window.clone();
            criterion["reference_and_uncertainty"] = json!(format!(
                "{} Negative case: {}",
                case.positive, case.negative
            ));
            criterion["proposed_limit"] = if correctness {
                json!({"rule":"exact_case_expected_outcomes","tolerance":0,"note":"Not a universal zero-loss requirement; declared rejections and protected durability boundary retained."})
            } else {
                Value::Null
            };
            criterion["proposal_note"] = json!(if cid == "inference_impact" {
                "No real inference workload; not-applicable decision/owner remains unagreed, so do not claim a pass."
            } else {
                "Numeric performance limits and agreements remain unset; no acceptance pass."
            });
        }
        outputs.push((path, encoded(&budget)?));
        experiments.push(experiment);
    }
    let registry = json!({"schema_version":1,"evidence_bundle":"aether-source-evidence-v1","environment_id":"aether-validation-synthetic-local-v1","experiments":experiments});
    let mut metrics = Vec::new();
    let references = |sources: BTreeSet<String>| json!({"pinned_upstream_ids":sources,"frozen_local_ids":["B01","B02","B03","B05","B06"],"runtime_evidence":[]});
    for req in array(&b.catalog, "metrics")? {
        let id = string(req, "id")?;
        let mapping = mappings
            .iter()
            .find(|r| r["id"] == id)
            .ok_or_else(|| error(id, "missing mapping"))?;
        let links: Vec<_> = experiments
            .iter()
            .filter(|e| {
                e["requirement_ids"]
                    .as_array()
                    .is_some_and(|a| a.contains(&json!(id)))
            })
            .collect();
        let source_ids = links
            .iter()
            .flat_map(|e| strings(&e["pinned_source_ids"]).unwrap_or_default())
            .collect();
        let selected = !links.is_empty();
        metrics.push(json!({"id":id,"baseline_requirement":req,"research_mapping":mapping,"selected_for_fixture":selected,"experiment_ids":links.iter().map(|e|e["id"].clone()).collect::<Vec<_>>(),"prototype_state":if selected {"fixture_planned"}else{"not_prototyped"},"evidence_references":references(source_ids),"remaining_unproven":[mapping["gap"].clone(),json!("Source collection, schema/capability, attribution and runtime experiment behavior remain unverified.")],"synthetic_execution_status":"not_run","deployed_schema_verification":"not_run","capability_verification":"not_run"}));
    }
    let mut associations = Vec::new();
    for a in &b.associations {
        let id = string(a, "id")?;
        let links: Vec<_> = experiments
            .iter()
            .filter(|e| {
                e["association_ids"]
                    .as_array()
                    .is_some_and(|v| v.contains(&json!(id)))
            })
            .collect();
        let source_ids = links
            .iter()
            .flat_map(|e| strings(&e["pinned_source_ids"]).unwrap_or_default())
            .collect();
        associations.push(json!({"id":id,"baseline_association":a,"selected_for_fixture":!links.is_empty(),"experiment_ids":links.iter().map(|e|e["id"].clone()).collect::<Vec<_>>(),"prototype_state":if links.is_empty(){"not_prototyped"}else{"fixture_planned"},"evidence_references":references(source_ids),"remaining_unproven":[a["cells"][4].clone(),a["cells"][5].clone(),json!("Invented associations do not prove real discovery or ownership.")],"synthetic_execution_status":"not_run","association_verification":"not_run","deployed_schema_verification":"not_run","capability_verification":"not_run"}));
    }
    metrics.sort_by_key(|v| v["id"].as_str().unwrap_or("").to_owned());
    associations.sort_by_key(|v| v["id"].as_str().unwrap_or("").to_owned());
    let selected_metrics = metrics
        .iter()
        .filter(|m| m["selected_for_fixture"] == true)
        .count();
    let selected_assocs = associations
        .iter()
        .filter(|m| m["selected_for_fixture"] == true)
        .count();
    let status = json!({"schema_version":1,"evidence_bundle":"aether-source-evidence-v1","evidence_manifest_sha256":manifest_hash,"environment_id":"aether-validation-synthetic-local-v1","experiment_registry":"docs/prototype-experiments.json","metrics":metrics,"associations":associations});
    let mut scope = format!(
        "# Prototype experiment scope — Milestone 1\n\nThis is a local Rust proof-of-concept program using invented inputs. It retains all 250 metric requirements across four layers and all 58 metadata associations. There is no existing cluster. No real engines, devices, runtime sockets, kernel probes, external storage or customer workloads are accessed. Production architecture is not selected. DeepInfer remains deferred.\n\nThe authoritative registry is [prototype-experiments.json](prototype-experiments.json); [prototype-status.json](prototype-status.json) preserves every baseline object and research route. {selected_metrics} metrics and {selected_assocs} associations are selected for planned fixtures; the remaining {} metrics and {} associations remain not_prototyped, not unsupported. Selection is bidirectional and dependencies remain visible.\n\n## Planned cases\n\n| ID | Purpose | Later execution milestones |\n|---|---|---|\n",
        250 - selected_metrics,
        58 - selected_assocs
    );
    for e in &experiments {
        scope.push_str(&format!(
            "| {} | {} | {} |\n",
            e["id"].as_str().unwrap_or(""),
            e["title"].as_str().unwrap_or(""),
            e["execution_milestones"]
        ));
    }
    scope.push_str("\n## Inputs and limits\n\nUse aether-source-evidence-v1 and aether-validation-synthetic-local-v1. Software references E01–E10 are exact repository snapshots, not a deployed stack. Host/device/fabric/semantic records are explicitly baseline-only synthetic unless new evidence is separately frozen. All six named serving sources are eligible; reviewing three engines first does not exclude others. Cache providers not independently verified remain unknown.\n\nL1-001 retains U because raw TGI request_count does not establish accepted inference requests; L1-005 inherits the gap. L2-071's raw endpoint exists but its skipped/speculated-token interpretation remains conditional. Current routes E:88/R:35/H:100/I:26/U:1 are not implementation or support counts. The historical 26 U rows are distinct from 26 I integration rows.\n\nThe twelve budget records are proposals, not agreed runtime limits. Correctness limits reference exact named outcomes; numeric production budgets and owner agreements remain unset. Readiness flags remain false; all experiment results and real verification remain not_run. The additional 100-ms proposed poll period, two retries and one-second capped backoff are small synthetic planning choices, not resolution/freshness guarantees. PX12 has a two-record reject-new queue assertion. Sensitive values and real prompt/response data are excluded.\n\n## Acceptance and handoff\n\nMilestone 1 validates planning records only. Run the Rust evidence and traceability commands and cargo tests under tools/aether-validation. A specification pass does not mean its experiment passed. Milestone 2 will create invented endpoint, metadata, event and fault payloads with expected answers. Milestone 3 adds the separate engine harness. Later source qualification, actual GPU/NIC/fabric support, backwards compatibility, microsecond acquisition, inference overhead, production durability and release remain unproven. Generic pipeline/policy checks and 18 proposed companion families never increase the original 250 metric IDs.\n");
    outputs.push(("docs/prototype-scope.md".into(), scope.into_bytes()));
    outputs.push((
        "docs/prototype-experiments.json".into(),
        encoded(&registry)?,
    ));
    outputs.push(("docs/prototype-status.json".into(), encoded(&status)?));
    write_outputs(root, outputs)
}
