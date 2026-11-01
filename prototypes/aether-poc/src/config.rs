use crate::{PrototypeError, Result, endpoint::ServerConfig};
use std::{collections::BTreeMap, path::PathBuf, time::Duration};

pub const MANIFEST: &str = "fixtures/prototype/manifest.json";
pub const HELP: &str = "aether-poc <validate-fixtures|list-fixtures|serve-fixture|run-scenario|verify-scenario|verify-milestone> [--root PATH] [--manifest fixtures/prototype/manifest.json]\nserve-fixture: --variant ID [--bind 127.0.0.1] [--port 0] [--max-requests 1] [--max-run-ms 5000]\nrun-scenario: --scenario NAME --variant ID\nverify-scenario: --scenario NAME --variant ID; verify-milestone: --milestone 4|5\nScenarios: endpoint-roundtrip, native-parse, counter-rate, aggregate-evidence, count-boundary, memory-occupancy, visibility-policy, freshness-evidence. M5 scenarios: publisher-identity, process-lifecycle, resource-membership, fabric-context, visibility-enrichment. Boundary queries use --scenario enrichment-boundaries --case E5-01 through E5-07.\nOnly owned loopback endpoints and synthetic records are used. Expected rejections and deferred stages remain explicit. Functional checks do not grant runtime acceptance.";
#[derive(Debug)]
pub enum Action {
    Help,
    Validate,
    List,
    Serve {
        variant: String,
        config: ServerConfig,
    },
    Run {
        variant: String,
        scenario: String,
    },
    Verify {
        variant: String,
        scenario: crate::scenario::MetricScenario,
    },
    VerifyMilestone,
    VerifyEnrichmentMilestone,
    VerifyEnrichment {
        variant: String,
        scenario: crate::enrichment_scenario::EnrichmentScenario,
    },
    Boundary {
        case_id: String,
        verify: bool,
    },
}
#[derive(Debug)]
pub struct Command {
    pub root: PathBuf,
    pub manifest: PathBuf,
    pub action: Action,
}
fn usage(s: impl ToString) -> PrototypeError {
    PrototypeError::new("usage", s)
}
pub fn parse_args(args: &[String]) -> Result<Command> {
    if args == ["--help"] || args == ["help"] {
        return Ok(Command {
            root: PathBuf::new(),
            manifest: MANIFEST.into(),
            action: Action::Help,
        });
    }
    let name = args
        .first()
        .ok_or_else(|| usage("missing command; use --help"))?;
    if ["query", "collect", "replay"].contains(&name.as_str()) {
        return Err(PrototypeError::new("not_prototyped", name));
    }
    if ![
        "validate-fixtures",
        "list-fixtures",
        "serve-fixture",
        "run-scenario",
        "verify-scenario",
        "verify-milestone",
    ]
    .contains(&name.as_str())
    {
        return Err(usage("unknown command"));
    }
    let mut values = BTreeMap::new();
    let mut iter = args[1..].iter();
    while let Some(flag) = iter.next() {
        let allowed = matches!(flag.as_str(), "--root" | "--manifest")
            || (name == "serve-fixture"
                && matches!(
                    flag.as_str(),
                    "--variant" | "--bind" | "--port" | "--max-requests" | "--max-run-ms"
                ))
            || (["run-scenario", "verify-scenario"].contains(&name.as_str())
                && matches!(flag.as_str(), "--variant" | "--scenario" | "--case"))
            || (name == "verify-milestone" && flag == "--milestone");
        if !allowed {
            return Err(usage(format!("unknown flag {flag}")));
        }
        let value = iter
            .next()
            .filter(|v| !v.starts_with("--"))
            .ok_or_else(|| usage(format!("missing value {flag}")))?;
        if values.insert(flag.as_str(), value.as_str()).is_some() {
            return Err(usage(format!("duplicate {flag}")));
        }
    }
    let get = |key| values.get(key).copied();
    let required = |key| {
        get(key)
            .map(str::to_string)
            .ok_or_else(|| usage(format!("required {key}")))
    };
    if get("--manifest").unwrap_or(MANIFEST) != MANIFEST {
        return Err(usage("only the schema-v1 repository manifest is supported"));
    }
    let number = |key, default| -> Result<u64> {
        let s = get(key).unwrap_or(default);
        if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
            return Err(usage(format!("invalid number {key}")));
        }
        s.parse()
            .map_err(|_| usage(format!("invalid number {key}")))
    };
    let action = match name.as_str() {
        "validate-fixtures" => Action::Validate,
        "list-fixtures" => Action::List,
        "serve-fixture" => {
            let config = ServerConfig {
                bind: get("--bind").unwrap_or("127.0.0.1").into(),
                port: u16::try_from(number("--port", "0")?)
                    .map_err(|_| usage("port out of range"))?,
                max_requests: number("--max-requests", "1")? as usize,
                max_run: Duration::from_millis(number("--max-run-ms", "5000")?),
            };
            config.validate().map_err(usage)?;
            Action::Serve {
                variant: required("--variant")?,
                config,
            }
        }
        "run-scenario" | "verify-scenario" => {
            let scenario = required("--scenario")?;
            if scenario == "enrichment-boundaries" {
                if get("--variant").is_some() {
                    return Err(usage("boundary scenarios require --case only"));
                }
                let case_id = required("--case")?;
                if !(1..=7).any(|i| case_id == format!("E5-{i:02}")) {
                    return Err(usage("unknown boundary case"));
                }
                Action::Boundary {
                    case_id,
                    verify: name == "verify-scenario",
                }
            } else {
                if get("--case").is_some() {
                    return Err(usage("only boundary scenarios accept --case"));
                }
                let enrichment = scenario
                    .parse::<crate::enrichment_scenario::EnrichmentScenario>()
                    .ok();
                if scenario != "endpoint-roundtrip" && enrichment.is_none() {
                    let _: crate::scenario::MetricScenario = scenario.parse()?;
                }
                if name == "verify-scenario" {
                    if let Some(scenario) = enrichment {
                        Action::VerifyEnrichment {
                            variant: required("--variant")?,
                            scenario,
                        }
                    } else {
                        Action::Verify {
                            variant: required("--variant")?,
                            scenario: scenario.parse()?,
                        }
                    }
                } else {
                    Action::Run {
                        variant: required("--variant")?,
                        scenario,
                    }
                }
            }
        }
        "verify-milestone" => {
            let milestone = required("--milestone")?;
            if milestone != "4" && milestone != "5" {
                return Err(PrototypeError::new(
                    "not_prototyped",
                    "only milestones 4 and 5 verification are implemented",
                ));
            }
            if milestone == "5" {
                Action::VerifyEnrichmentMilestone
            } else {
                Action::VerifyMilestone
            }
        }
        _ => unreachable!(),
    };
    let root = PathBuf::from(get("--root").unwrap_or(".")).canonicalize()?;
    Ok(Command {
        root,
        manifest: MANIFEST.into(),
        action,
    })
}
