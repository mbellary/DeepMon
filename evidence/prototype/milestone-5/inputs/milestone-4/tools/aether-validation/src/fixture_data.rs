//! Explicit invented inputs and independent literal oracles; not engine output.
use crate::fixtures::*;
use crate::*;
use serde_json::json;
struct Draft {
    name: &'static str,
    intent: &'static str,
    records: Vec<Value>,
    assertions: Vec<FixtureAssertion>,
    proof: Option<Value>,
    endpoint: Option<String>,
    invalid: Option<&'static str>,
    remote_clock: bool,
}
fn assertion(
    kind: &str,
    target: &str,
    value: Value,
    unit: &str,
    state: &str,
    ids: &[&str],
    reason: &str,
) -> FixtureAssertion {
    FixtureAssertion {
        kind: kind.into(),
        target: target.into(),
        expected_value: value,
        unit: unit.into(),
        state: state.into(),
        record_ids: ids.iter().map(|s| s.to_string()).collect(),
        reason: reason.into(),
    }
}
fn draft(
    name: &'static str,
    negative: bool,
    records: Vec<Value>,
    assertions: Vec<FixtureAssertion>,
) -> Draft {
    Draft {
        name,
        intent: if negative { "negative" } else { "positive" },
        records,
        assertions,
        proof: None,
        endpoint: None,
        invalid: None,
        remote_clock: false,
    }
}
fn unknown(target: &str, reason: &str) -> FixtureAssertion {
    assertion(
        "unknown",
        target,
        Value::Null,
        "not_available",
        "unknown",
        &[],
        reason,
    )
}
fn identity(id: &str, entity: &str, incarnation: &str, from: u64, to: u64) -> Value {
    json!({"id":id,"kind":"identity","entity":entity,"incarnation":incarnation,"valid_from_us":from,"valid_to_us":to,"evidence":"invented_explicit_membership"})
}
fn counter(id: &str, family: &str, value: u64, time: u64, inc: &str) -> Value {
    json!({"id":id,"kind":"counter","family":family,"value":value,"time_us":time,"clock_domain":"fixture","incarnation":inc,"unit":"requests"})
}
fn endpoint_record(
    producer: &str,
    family: &str,
    typ: &str,
    unit: &str,
    sources: &[&str],
    scope: &str,
) -> Value {
    json!({"id":"snapshot","kind":"endpoint_snapshot","producer":producer,"family":family,"metric_type":typ,"unit":unit,"scope":scope,"source_input_ids":sources,"time_us":0,"clock_domain":"fixture","invented_fields":["values","model_alias","fixture_bucket_boundaries"],"native_labels":[],"invented_labels":[]})
}
fn prom(family: &str, typ: &str, labels: &str, value: u64) -> String {
    let labelpart = if labels.is_empty() {
        String::new()
    } else {
        format!("{{{labels}}}")
    };
    let base = format!(
        "# HELP {family} Synthetic test values; not a captured export\n# TYPE {family} {typ}\n"
    );
    if typ == "histogram" {
        let comma = if labels.is_empty() {
            String::new()
        } else {
            format!(",{labels}")
        };
        format!(
            "{base}{family}_bucket{{le=\"1\"{comma}}} 1\n{family}_bucket{{le=\"+Inf\"{comma}}} 2\n{family}_count{labelpart} 2\n{family}_sum{labelpart} 3\n"
        )
    } else {
        format!("{base}{family}{labelpart} {value}\n")
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Keep native producer/type/unit/provenance fields explicit in static test data"
)]
fn endpoint(
    name: &'static str,
    producer: &str,
    family: &str,
    typ: &str,
    unit: &str,
    source: &[&str],
    scope: &str,
    labels: &str,
) -> Draft {
    let mut r = endpoint_record(producer, family, typ, unit, source, scope);
    r["native_labels"] = json!(if labels.is_empty() {
        vec![]
    } else {
        labels
            .split(',')
            .filter_map(|p| p.split('=').next())
            .collect::<Vec<_>>()
    });
    let mut d = draft(
        name,
        false,
        vec![r],
        vec![assertion(
            "native",
            family,
            json!({"type":typ,"scope":scope,"sample_values":if typ=="histogram" {json!([1,2,2,3])}else{json!([2])}}),
            unit,
            "reported_only",
            &["snapshot"],
            "Retain the declared family/type/labels and producer scope; values and bucket boundaries are invented.",
        )],
    );
    d.endpoint = Some(prom(family, typ, labels, 2));
    d
}
fn data(px: &str) -> Vec<Draft> {
    match px {
        "PX01" => {
            let mut v = vec![
                endpoint(
                    "vllm",
                    "vllm",
                    "vllm:iteration_tokens_total",
                    "histogram",
                    "tokens",
                    &["E01", "E02"],
                    "engine",
                    "model_name=\"fixture-model\"",
                ),
                endpoint(
                    "sglang",
                    "sglang",
                    "sglang:e2e_request_latency_seconds",
                    "histogram",
                    "seconds",
                    &["E03", "E04"],
                    "engine",
                    "model_name=\"fixture-alias\"",
                ),
                endpoint(
                    "dynamo",
                    "dynamo_frontend",
                    "dynamo_frontend_request_duration_seconds",
                    "histogram",
                    "seconds",
                    &["E05"],
                    "frontend",
                    "model=\"fixture-model\"",
                ),
                endpoint(
                    "tgi",
                    "tgi",
                    "tgi_batch_current_size",
                    "gauge",
                    "sequences",
                    &["E06"],
                    "batch",
                    "",
                ),
                endpoint(
                    "triton",
                    "triton",
                    "nv_pinned_memory_pool_used_bytes",
                    "gauge",
                    "bytes",
                    &["E08"],
                    "server_all_models",
                    "",
                ),
                endpoint(
                    "trt-native",
                    "trtllm",
                    "trtllm_kv_cache_used_blocks",
                    "gauge",
                    "blocks",
                    &["E09"],
                    "engine_pool",
                    "",
                ),
                endpoint(
                    "trt-backend",
                    "triton_trtllm",
                    "nv_trt_llm_runtime_memory_metrics",
                    "gauge",
                    "bytes",
                    &["E10"],
                    "backend_runtime",
                    "memory_type=\"gpu\",model=\"fixture-model\",version=\"1\"",
                ),
            ];
            let mut bad = endpoint(
                "malformed-type",
                "tgi",
                "tgi_batch_current_size",
                "gauge",
                "sequences",
                &["E06"],
                "batch",
                "",
            );
            bad.intent = "negative";
            bad.endpoint = bad
                .endpoint
                .map(|p| p.replace(" gauge\n", " invented_type\n"));
            bad.invalid = Some("TYPE_INVALID");
            bad.assertions = vec![assertion(
                "rejection",
                "payload",
                json!("TYPE_INVALID"),
                "error_code",
                "rejected",
                &["snapshot"],
                "The bounded fixture parser must reject the actual invalid TYPE declaration.",
            )];
            v.push(bad);
            let mut hist = endpoint(
                "inconsistent-histogram",
                "vllm",
                "vllm:iteration_tokens_total",
                "histogram",
                "tokens",
                &["E02"],
                "engine",
                "",
            );
            hist.intent = "negative";
            hist.endpoint = hist.endpoint.map(|p| p.replace("_count 2", "_count 3"));
            hist.invalid = Some("HISTOGRAM_INCONSISTENT");
            hist.assertions = vec![assertion(
                "rejection",
                "payload",
                json!("HISTOGRAM_INCONSISTENT"),
                "error_code",
                "rejected",
                &["snapshot"],
                "Infinite bucket and total count disagree.",
            )];
            v.push(hist);
            v.push(draft("optional-absent",true,vec![json!({"id":"optional","kind":"gauge","value":0,"unit":"capability_state","family_present":false,"statistics_active":"unknown"})],vec![unknown("trt_iteration_latency","Optional family absent; no zero-valued duration or active-statistics claim.")]));
            v
        }
        "PX02" => {
            let mut good = draft(
                "success-rate",
                false,
                vec![
                    counter("c0", "nv_inference_request_success", 100, 0, "one"),
                    counter("c1", "nv_inference_request_success", 150, 10_000_000, "one"),
                ],
                vec![assertion(
                    "rate",
                    "source_success_rate",
                    json!(5),
                    "requests/second",
                    "known",
                    &["c0", "c1"],
                    "(150-100)/(10,000,000/1,000,000) = 5; this is request-success rate, not tokens.",
                )],
            );
            good.endpoint = Some(prom(
                "nv_inference_request_success",
                "counter",
                "model=\"fixture-model\",version=\"1\"",
                100,
            ));
            vec![
                good,
                draft(
                    "reset",
                    true,
                    vec![
                        counter("c0", "nv_inference_request_success", 100, 0, "one"),
                        counter("c1", "nv_inference_request_success", 20, 10_000_000, "two"),
                    ],
                    vec![unknown(
                        "source_success_rate",
                        "New incarnation/reset with unknown reset instant prevents an exact rate; never negative.",
                    )],
                ),
                draft(
                    "raw-tgi-boundary",
                    true,
                    vec![counter("raw", "tgi_request_count", 150, 10_000_000, "one")],
                    vec![unknown(
                        "accepted_arrival_rate",
                        "L1-001 and L1-005 remain unresolved despite a readable raw counter.",
                    )],
                ),
            ]
        }
        "PX03" => {
            let aggregates = json!({"id":"hist","kind":"aggregate","unit":"seconds","buckets":[[1,2],["+Inf",2]],"count":2,"sum":1,"success_count":1,"missing_joint_records":true});
            let mut d = draft(
                "aggregate-proof",
                false,
                vec![aggregates.clone()],
                vec![assertion(
                    "proof",
                    "counterexamples",
                    json!({"variance_a":0.0625,"variance_b":0,"goodput_a":0.1,"goodput_b":0}),
                    "mixed_explicit_proof_units",
                    "known",
                    &["hist"],
                    "Hypothetical oracle-only worlds have equal aggregates and different answers; no hidden events are supplied as runtime input.",
                )],
            );
            d.proof = Some(
                json!({"variance":{"a":[0.25,0.75],"b":[0.5,0.5],"buckets_seconds":[1,"+Inf"],"unit":"seconds_squared","population":true},"goodput":{"latencies_seconds":[0.2,2.0],"success_a":[true,false],"success_b":[false,true],"slo_seconds":1,"window_seconds":10,"unit":"requests/second"}}),
            );
            vec![
                d,
                draft(
                    "insufficient-joint-data",
                    true,
                    vec![aggregates],
                    vec![unknown(
                        "exact_goodput_and_variance",
                        "Marginal count/sum/buckets and separate success count do not establish joint qualification or exact variance.",
                    )],
                ),
            ]
        }
        "PX04" => vec![
            draft(
                "overlap-and-remote",
                false,
                vec![
                    json!({"id":"front","kind":"aggregate","producer":"frontend","logical_ids":["r0","r1","r2","r3","r4","r5","r6","r7","r8","r9"],"count":10}),
                    json!({"id":"back","kind":"aggregate","producer":"backend","logical_ids":["r0","r1","r2","r3","r4","r5","r6","r7","r8","r9"],"count":10}),
                    identity("front-host", "frontend-host", "boot-one", 0, 20_000_000),
                    json!({"id":"worker","kind":"link","emitter":"frontend","subject":"remote-worker","subject_host":null,"evidence":"location_missing"}),
                ],
                vec![
                    assertion(
                        "logical_count",
                        "selected_boundary",
                        json!(10),
                        "requests",
                        "known",
                        &["front", "back"],
                        "Both invented lists describe the same ten logical IDs, not twenty requests.",
                    ),
                    unknown(
                        "worker_host",
                        "Frontend location is not remote-worker location.",
                    ),
                ],
            ),
            draft(
                "ambiguous-publisher",
                true,
                vec![
                    identity("p1", "process-A", "start-one", 0, 20_000_000),
                    identity("p2", "process-B", "start-two", 0, 20_000_000),
                    json!({"id":"listener","kind":"link","publisher_candidates":["p1","p2"],"model_alias":"same-model"}),
                ],
                vec![assertion(
                    "state",
                    "publisher",
                    json!(["p1", "p2"]),
                    "candidate_set",
                    "ambiguous",
                    &["p1", "p2", "listener"],
                    "Retain both valid candidates; no first-PID or model-name ownership inference.",
                )],
            ),
        ],
        "PX05" => vec![
            draft(
                "replica-set",
                false,
                vec![
                    identity("replica", "replica-A", "generation-one", 0, 20_000_000),
                    identity(
                        "other-replica",
                        "replica-B",
                        "generation-two",
                        0,
                        20_000_000,
                    ),
                    json!({"id":"members","kind":"link","replica":"replica-A","members":["gpu-A","gpu-B"],"models":["symbolic-model-A","symbolic-model-B"],"other_replica_same_model_alias":"replica-B","complete":true}),
                    json!({"id":"replica-count","kind":"aggregate","count":100,"scope":"replica-A"}),
                    json!({"id":"mA","kind":"memory","device":"gpu-A","used":10,"free":10,"total":20,"unit":"MiB"}),
                    json!({"id":"mB","kind":"memory","device":"gpu-B","used":20,"free":20,"total":40,"unit":"MiB"}),
                ],
                vec![
                    assertion(
                        "replica_count",
                        "replica-A",
                        json!(100),
                        "requests",
                        "known",
                        &["replica-count", "members"],
                        "One replica observation references a two-member set; it is not copied per GPU.",
                    ),
                    assertion(
                        "occupancy",
                        "gpu-A",
                        json!(0.5),
                        "ratio",
                        "known",
                        &["mA"],
                        "10/20 = 0.5.",
                    ),
                    assertion(
                        "occupancy",
                        "gpu-B",
                        json!(0.5),
                        "ratio",
                        "known",
                        &["mB"],
                        "20/40 = 0.5.",
                    ),
                ],
            ),
            draft(
                "partial-membership",
                true,
                vec![
                    json!({"id":"partial","kind":"link","replica":"replica-A","members":["gpu-A"],"complete":false}),
                    json!({"id":"missing-total","kind":"gauge","value":10,"unit":"MiB","total":null}),
                ],
                vec![
                    unknown(
                        "gpu_occupancy",
                        "Missing total prevents occupancy; used memory is not total capacity.",
                    ),
                    assertion(
                        "state",
                        "composition",
                        json!(["gpu-A"]),
                        "device_set",
                        "partial",
                        &["partial"],
                        "Do not invent missing members or one universal GPU model.",
                    ),
                ],
            ),
        ],
        "PX06" => vec![
            draft(
                "pid-reuse",
                false,
                vec![
                    identity("old", "host-boot/PID17", "start-one", 0, 10),
                    identity("new", "host-boot/PID17", "start-two", 10, 20),
                    json!({"id":"old-membership","kind":"link","process_record":"old","cgroup":"cgroup-A","container":"container-A","valid_from_us":0,"valid_to_us":10}),
                    json!({"id":"new-membership","kind":"link","process_record":"new","cgroup":"cgroup-B","container":"container-B","valid_from_us":10,"valid_to_us":20}),
                ],
                vec![assertion(
                    "state",
                    "historical_owner",
                    json!("container-A"),
                    "container",
                    "resolved",
                    &["old", "old-membership"],
                    "New PID incarnation and moved cgroup do not relabel earlier records.",
                )],
            ),
            draft(
                "runtime-denied",
                true,
                vec![
                    identity("proc", "host-boot/PID17", "start-two", 10, 20),
                    json!({"id":"denied","kind":"policy","source":"runtime_metadata","status":"permission_denied","container":null}),
                ],
                vec![unknown(
                    "container",
                    "Read denial is not proof of no container and cannot cause a guessed owner.",
                )],
            ),
        ],
        "PX07" => vec![
            draft(
                "inventory-only",
                false,
                vec![
                    identity("nic", "fixture-netns/ifindex7", "link-one", 0, 20_000_000),
                    json!({"id":"capability","kind":"link","model":"symbolic-NIC","capabilities":["ethernet","rdma"],"configured_preference":"roce","operation_path":null,"driver":"synthetic-driver","loaded_cuda_nccl":null}),
                    json!({"id":"bytes","kind":"gauge","value":1024,"unit":"bytes","scope":"fixture-netns/ifindex7"}),
                ],
                vec![
                    assertion(
                        "state",
                        "capabilities",
                        json!(["ethernet", "rdma"]),
                        "capability_set",
                        "reported_only",
                        &["capability"],
                        "Capability and configuration are distinct from observed traffic path.",
                    ),
                    unknown(
                        "active_transport",
                        "No operation path evidence proves RoCE, InfiniBand or TCP usage.",
                    ),
                ],
            ),
            draft(
                "control-socket-only",
                true,
                vec![
                    json!({"id":"control","kind":"link","socket":"symbolic-control-peer","protocol":"tcp","is_payload_path":null}),
                ],
                vec![unknown(
                    "operation_to_nic",
                    "One control socket cannot identify collective payload transport/NIC.",
                )],
            ),
        ],
        "PX08" => {
            let events = vec![
                json!({"id":"rank0","kind":"rank_event","group":"group-A","generation":"one","rank":0,"arrival_us":100,"completion_us":150,"clock_domain":"fixture"}),
                json!({"id":"rank1","kind":"rank_event","group":"group-A","generation":"one","rank":1,"arrival_us":130,"completion_us":190,"clock_domain":"fixture"}),
                json!({"id":"group","kind":"link","expected_ranks":[0,1],"complete":true}),
            ];
            let good = draft(
                "complete-group",
                false,
                events.clone(),
                vec![
                    assertion(
                        "arrival_skew",
                        "group-A",
                        json!(30),
                        "microseconds",
                        "known",
                        &["rank0", "rank1", "group"],
                        "130-100 on one exact invented clock.",
                    ),
                    assertion(
                        "completion_skew",
                        "group-A",
                        json!(40),
                        "microseconds",
                        "known",
                        &["rank0", "rank1", "group"],
                        "190-150 on one exact invented clock.",
                    ),
                    assertion(
                        "duration",
                        "rank0",
                        json!(50),
                        "microseconds",
                        "known",
                        &["rank0"],
                        "150-100.",
                    ),
                    assertion(
                        "duration",
                        "rank1",
                        json!(60),
                        "microseconds",
                        "known",
                        &["rank1"],
                        "190-130.",
                    ),
                ],
            );
            let missing = draft(
                "missing-rank",
                true,
                vec![
                    events[0].clone(),
                    json!({"id":"group","kind":"link","expected_ranks":[0,1],"complete":false}),
                ],
                vec![unknown(
                    "whole_group_skew",
                    "Missing expected participant prevents whole-group comparison.",
                )],
            );
            let mut changed = events.clone();
            changed[1]["generation"] = json!("two");
            let mismatch = draft(
                "group-mismatch",
                true,
                changed,
                vec![unknown(
                    "whole_group_skew",
                    "Rank records belong to different group incarnations.",
                )],
            );
            let mut clocks = events;
            clocks[1]["clock_domain"] = json!("remote");
            let mut remote = draft(
                "unknown-clock-offset",
                true,
                clocks,
                vec![unknown(
                    "whole_group_skew",
                    "Remote clock offset is unknown; no exact cross-clock skew.",
                )],
            );
            remote.remote_clock = true;
            vec![good, missing, mismatch, remote]
        }
        "PX09" => vec![
            draft(
                "overlapping-streams",
                false,
                vec![
                    json!({"id":"A0","kind":"interval","stream":"A","start_us":0,"end_us":10,"clock_domain":"fixture"}),
                    json!({"id":"A1","kind":"interval","stream":"A","start_us":20,"end_us":30,"clock_domain":"fixture"}),
                    json!({"id":"B0","kind":"interval","stream":"B","start_us":10,"end_us":20,"clock_domain":"fixture"}),
                    json!({"id":"coverage","kind":"link","complete_stream_coverage":true,"window_us":[0,30]}),
                ],
                vec![
                    assertion(
                        "stream_gap",
                        "A",
                        json!(10),
                        "microseconds",
                        "known",
                        &["A0", "A1"],
                        "Half-open A intervals have a ten-microsecond gap.",
                    ),
                    assertion(
                        "device_idle",
                        "device",
                        json!(0),
                        "microseconds",
                        "known",
                        &["A0", "A1", "B0", "coverage"],
                        "The supplied union covers [0,30); this proves only invented coverage.",
                    ),
                ],
            ),
            draft(
                "incomplete-stream-coverage",
                true,
                vec![
                    json!({"id":"A0","kind":"interval","stream":"A","start_us":0,"end_us":10,"clock_domain":"fixture"}),
                    json!({"id":"A1","kind":"interval","stream":"A","start_us":20,"end_us":30,"clock_domain":"fixture"}),
                    json!({"id":"coverage","kind":"link","complete_stream_coverage":false,"window_us":[0,30]}),
                ],
                vec![unknown(
                    "device_idle",
                    "Missing stream coverage does not prove device-wide idle time.",
                )],
            ),
        ],
        "PX10" => vec![
            draft(
                "protected-replay",
                false,
                vec![
                    json!({"id":"r0","kind":"stage","logical_id":"r0","admitted":true,"persisted":true,"committed":true,"query_visible":true}),
                    json!({"id":"r1","kind":"stage","logical_id":"r1","admitted":true,"persisted":true,"committed":true,"query_visible":true}),
                    json!({"id":"retry","kind":"fault","action":"duplicate_retry","logical_id":"r0","after":"persisted_before_commit","sequence":3}),
                ],
                vec![assertion(
                    "committed_ids",
                    "snapshot",
                    json!(["r0", "r1"]),
                    "record_ids",
                    "known",
                    &["r0", "r1"],
                    "Protected simulated boundary retains two unique records; retry does not create a third.",
                )],
            ),
            draft(
                "volatile-loss",
                true,
                vec![
                    json!({"id":"r0","kind":"stage","logical_id":"r0","admitted":true,"persisted":true,"committed":true,"query_visible":true}),
                    json!({"id":"r1","kind":"stage","logical_id":"r1","admitted":true,"persisted":false,"committed":false,"query_visible":false}),
                    json!({"id":"loss","kind":"fault","action":"volatile_loss","logical_id":"r1","after":"admitted_before_persisted","sequence":2}),
                ],
                vec![
                    assertion(
                        "committed_ids",
                        "snapshot",
                        json!(["r0"]),
                        "record_ids",
                        "known",
                        &["r0", "r1"],
                        "Volatile r1 can be lost; no universal zero-loss claim.",
                    ),
                    assertion(
                        "state",
                        "lost_ids",
                        json!(["r1"]),
                        "record_ids",
                        "reported_only",
                        &["r1", "loss"],
                        "Loss is explicitly accounted for.",
                    ),
                ],
            ),
            draft(
                "uncommitted-invisible",
                true,
                vec![
                    json!({"id":"r0","kind":"stage","logical_id":"r0","admitted":true,"persisted":true,"committed":true,"query_visible":true}),
                    json!({"id":"r1","kind":"stage","logical_id":"r1","admitted":true,"persisted":true,"committed":false,"query_visible":false}),
                    json!({"id":"commit-failure","kind":"fault","action":"commit_visibility_interruption","after":"persisted_before_commit","sequence":3}),
                    json!({"id":"write-failure","kind":"fault","action":"write_failure","after":"before_next_write","sequence":4}),
                ],
                vec![assertion(
                    "visible_ids",
                    "snapshot",
                    json!(["r0"]),
                    "record_ids",
                    "known",
                    &["r0", "r1"],
                    "Persisted but uncommitted r1 is excluded from the query snapshot.",
                )],
            ),
        ],
        "PX11" => vec![
            draft(
                "visibility-and-redaction",
                false,
                vec![
                    json!({"id":"visibility","kind":"policy","source_status":"permission_denied","optional_family":"not_enabled","identity":null,"synthetic_sensitive_marker":"SYNTHETIC_ONLY_SECRET_MARKER","retained_labels":{"fixture":"safe"}}),
                ],
                vec![
                    assertion(
                        "sanitized",
                        "retained_labels",
                        json!({"fixture":"safe"}),
                        "label_map",
                        "known",
                        &["visibility"],
                        "No synthetic sensitive marker survives in expected retained labels.",
                    ),
                    unknown(
                        "identity",
                        "Denied visibility and disabled family are not healthy zeros.",
                    ),
                ],
            ),
            draft(
                "forbidden-actions",
                true,
                vec![
                    json!({"id":"actions","kind":"policy","attempted_actions":["enable_metric","escalate_privilege","restart_workload"],"required_limit":null,"agreement":null,"runtime_readiness":false}),
                ],
                vec![
                    assertion(
                        "state",
                        "actions",
                        json!("rejected"),
                        "policy_decision",
                        "denied",
                        &["actions"],
                        "All forbidden actions remain data; no real activation occurs.",
                    ),
                    assertion(
                        "state",
                        "runtime_readiness",
                        json!(false),
                        "boolean",
                        "blocked",
                        &["actions"],
                        "Required-null limit and absent agreement block runtime acceptance.",
                    ),
                ],
            ),
        ],
        "PX12" => vec![
            draft(
                "gauge-and-queue",
                false,
                vec![
                    json!({"id":"g0","kind":"gauge","family":"trtllm_iteration_latency_seconds","value":0.001,"unit":"seconds","time_us":0,"clock_domain":"fixture"}),
                    json!({"id":"g1","kind":"gauge","family":"trtllm_iteration_latency_seconds","value":0.001,"unit":"seconds","time_us":10_000_000,"clock_domain":"fixture"}),
                    json!({"id":"q","kind":"queue","capacity_records":2,"policy":"reject_new","offered_ids":["q0","q1","q2"],"admitted_ids":["q0","q1"],"rejected_ids":["q2"]}),
                ],
                vec![
                    assertion(
                        "queue_counts",
                        "q",
                        json!({"offered":3,"admitted":2,"rejected":1}),
                        "records",
                        "known",
                        &["q"],
                        "Two-record reject-new queue explicitly rejects q2.",
                    ),
                    unknown(
                        "intervening_source_updates",
                        "Equal latest gauges do not imply no source updates or complete iteration capture.",
                    ),
                ],
            ),
            draft(
                "unknown-persistence-freshness",
                true,
                vec![
                    json!({"id":"delay","kind":"link","capture_time_us":0,"persisted_time_us":null,"query_visible_time_us":null,"source_update_time_us":null}),
                ],
                vec![unknown(
                    "source_and_query_freshness",
                    "Missing source, persistence and visibility times cannot be filled from poll interval.",
                )],
            ),
        ],
        _ => Vec::new(),
    }
}
fn encoded<T: serde::Serialize>(v: &T) -> ValidationResult<Vec<u8>> {
    let mut b = serde_json::to_vec_pretty(v).map_err(|e| error("fixture serialization", e))?;
    b.push(b'\n');
    Ok(b)
}
fn artifact(
    outputs: &mut Vec<(String, Vec<u8>)>,
    path: String,
    role: &str,
    media: &str,
    bytes: Vec<u8>,
) -> FixtureArtifact {
    let a = FixtureArtifact {
        path: path.clone(),
        role: role.into(),
        media_type: media.into(),
        bytes: bytes.len() as u64,
        sha256: sha256(&bytes),
    };
    outputs.push((format!("fixtures/prototype/{path}"), bytes));
    a
}
pub fn author_fixtures(root: &Path) -> ValidationResult<usize> {
    check_traceability(root)?;
    let registry = json(root, "docs/prototype-experiments.json")?;
    let mut outputs = Vec::new();
    let mut groups = Vec::new();
    for experiment in array(&registry, "experiments")? {
        let px = string(experiment, "id")?;
        let mut variants = Vec::new();
        for d in data(px) {
            let id = format!("{px}-{}-v1", d.name);
            let mut clocks = vec![FixtureClock {
                domain: "fixture".into(),
                unit: "microseconds".into(),
                uncertainty_us: 0,
                offset_bounds_us: Some([0, 0]),
            }];
            if d.remote_clock {
                clocks.push(FixtureClock {
                    domain: "remote".into(),
                    unit: "microseconds".into(),
                    uncertainty_us: 5,
                    offset_bounds_us: None,
                });
            }
            let input = FixtureInput {
                schema_version: 1,
                synthetic: true,
                variant_id: id.clone(),
                clocks,
                records: d.records,
            };
            let expected = FixtureExpected {
                schema_version: 1,
                synthetic: true,
                variant_id: id.clone(),
                experiment_execution_status: "not_run".into(),
                assertions: d.assertions,
                proof: d.proof,
            };
            let dir = match px {
                "PX01" | "PX02" | "PX03" => "endpoints",
                "PX04" | "PX05" | "PX06" | "PX07" => "metadata",
                "PX08" | "PX09" | "PX12" => "events",
                _ => "faults",
            };
            let mut artifacts = vec![
                artifact(
                    &mut outputs,
                    format!("{dir}/{id}.json"),
                    "input",
                    "application/json",
                    encoded(&input)?,
                ),
                artifact(
                    &mut outputs,
                    format!("expected/{id}.json"),
                    "expected",
                    "application/json",
                    encoded(&expected)?,
                ),
            ];
            if let Some(p) = d.endpoint {
                artifacts.push(artifact(
                    &mut outputs,
                    format!("endpoints/{id}.prom"),
                    "endpoint",
                    "text/plain",
                    p.into_bytes(),
                ));
            }
            variants.push(FixtureVariant {
                id,
                purpose: format!("{}: {}", string(experiment, "title")?, d.name),
                intent: d.intent.into(),
                expected_payload_error: d.invalid.map(str::to_owned),
                experiment_execution_status: "not_run".into(),
                artifacts,
            });
        }
        let vecstr = |key: &str| -> ValidationResult<Vec<String>> {
            Ok(strings(&experiment[key])?.into_iter().collect())
        };
        let budget_path = string(experiment, "budget_record")?.to_owned();
        groups.push(FixtureGroup{fixture_id:string(experiment,"planned_fixture_id")?.into(),experiment_id:px.into(),requirement_ids:vecstr("requirement_ids")?,association_ids:vecstr("association_ids")?,profile_ids:vecstr("profile_ids")?,pinned_source_ids:vecstr("pinned_source_ids")?,local_baseline_ids:vecstr("local_baseline_ids")?,evidence_kind:string(experiment,"evidence_kind")?.into(),invented_fields:"Every numeric value, ID, incarnation, timestamp, model alias, hypothetical clock, fixture bucket boundary and hardware model is invented; no captured data.".into(),limits_of_claim:string(experiment,"limits_of_claim")?.into(),budget_sha256:sha256(&read(root,&budget_path)?),budget_path,variants});
    }
    let manifest = FixtureManifest {
        schema_version: 1,
        bundle_id: "aether-synthetic-fixtures-v1".into(),
        synthetic: true,
        source_manifest_sha256: sha256(&read(root, "evidence/manifest-v1.json")?),
        registry_sha256: sha256(&read(root, "docs/prototype-experiments.json")?),
        status_sha256: sha256(&read(root, "docs/prototype-status.json")?),
        environment_id: "aether-validation-synthetic-local-v1".into(),
        fixtures: groups,
    };
    let mbytes = encoded(&manifest)?;
    let ledger = json!({"schema_version":1,"synthetic":true,"manifest_sha256":sha256(&mbytes),"registry_sha256":manifest.registry_sha256,"status_sha256":manifest.status_sha256,"runtime_acceptance_readiness":false,"deployed_schema_verification":"not_run","capability_verification":"not_run","entries":manifest.fixtures.iter().map(|g|json!({"experiment_id":g.experiment_id,"fixture_id":g.fixture_id,"variant_ids":g.variants.iter().map(|v|v.id.clone()).collect::<Vec<_>>(),"files":g.variants.iter().flat_map(|v|v.artifacts.iter().map(|a|a.path.clone())).collect::<Vec<_>>(),"authoring_status":"authored","fixture_validation_status":"not_run","fixture_validation_evidence":null,"experiment_execution_status":"not_run"})).collect::<Vec<_>>()});
    outputs.push(("fixtures/prototype/manifest.json".into(), mbytes));
    outputs.push((
        "docs/prototype-fixture-status.json".into(),
        encoded(&ledger)?,
    ));
    outputs.push(("fixtures/prototype/README.md".into(),b"# Invented observability fixtures v1\n\nAll inputs and expected answers are synthetic. No engine, host, device, fault adapter or network is executed here. Inspect manifest.json for twelve parent PX groups, local input/endpoint/expected files, byte hashes, source references and negative variants. Full format: ../../docs/prototype-fixture-format.md.\n\nAll timestamps are virtual microseconds with clock evidence; intervals are half-open. Oracle counterexamples are kept out of aggregate-only runtime inputs. Native family names follow pinned source definitions, but all values, aliases and test bucket boundaries are invented. Malformed exposition is useful test input only when its specific rejection is actually checked. Hardware identities are symbolic.\n\nRun cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- fixtures --root . from the workspace. A construction pass is not an engine-experiment pass, real schema/capability verification or runtime budget agreement. Source/registry/status/budget inputs remain accepted Milestone 1 bytes.\n".to_vec()));
    crate::generate::write_outputs(root, outputs)
}
