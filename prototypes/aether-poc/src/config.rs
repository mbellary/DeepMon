use crate::{PrototypeError, Result, endpoint::ServerConfig};
use std::{collections::BTreeMap, path::PathBuf, time::Duration};

pub const MANIFEST: &str = "fixtures/prototype/manifest.json";
pub const HELP: &str = "aether-poc <validate-fixtures|list-fixtures|serve-fixture|run-scenario> [--root PATH] [--manifest fixtures/prototype/manifest.json]\nserve-fixture: --variant ID [--bind 127.0.0.1] [--port 0] [--max-requests 1] [--max-run-ms 5000]\nrun-scenario: --scenario endpoint-roundtrip --variant ID\nOnly owned loopback endpoints are used. Standalone serving prints startup/shutdown JSON lines and exits after its connection limit or deadline. The scenario owns both ends and prints one deterministic report. Metric interpretation remains not_prototyped.";
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
            || (name == "run-scenario" && matches!(flag.as_str(), "--variant" | "--scenario"));
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
        "run-scenario" => {
            let scenario = required("--scenario")?;
            if scenario != "endpoint-roundtrip" {
                return Err(PrototypeError::new("not_prototyped", scenario));
            }
            Action::Run {
                variant: required("--variant")?,
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
