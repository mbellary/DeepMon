use aether_poc::{
    PrototypeError, Result,
    config::{self, Action},
    endpoint, fixture,
    report::FixtureListing,
};
use std::io::Write;
fn emit(value: &impl serde::Serialize) -> Result<()> {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let json = serde_json::to_string(value)
        .map_err(|_| PrototypeError::new("report_error", "serialization failed"))?;
    out.write_all(
        json.replace(aether_poc::observation::MARKER, "[REDACTED]")
            .as_bytes(),
    )?;
    writeln!(out)?;
    out.flush()?;
    Ok(())
}
fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let command = config::parse_args(&args)?;
    if matches!(command.action, Action::Help) {
        println!("{}", config::HELP);
        return Ok(());
    }
    let bundle = fixture::load_bundle(&command.root, &command.manifest)?;
    match command.action {
        Action::Help => unreachable!(),
        Action::Validate => emit(&FixtureListing::new(&bundle, false)),
        Action::List => emit(&FixtureListing::new(&bundle, true)),
        Action::Run { variant, scenario } if scenario == "endpoint-roundtrip" => emit(
            &endpoint::run_endpoint_roundtrip(fixture::select_endpoint(&bundle, &variant)?)?,
        ),
        Action::Run { variant, scenario } => emit(&aether_poc::scenario::run_metric_scenario(
            &bundle,
            scenario.parse()?,
            &variant,
        )?),
        Action::Verify { variant, scenario } => {
            let report = aether_poc::scenario::verify_metric_scenario(&bundle, scenario, &variant)?;
            emit(&report)?;
            if report.passed {
                Ok(())
            } else {
                Err(PrototypeError::new(
                    "verification_mismatch",
                    "required functional check failed",
                ))
            }
        }
        Action::VerifyMilestone => {
            let report = aether_poc::scenario::verify_milestone(&bundle)?;
            emit(&report)?;
            if report["functional_checks_passed"] == true {
                Ok(())
            } else {
                Err(PrototypeError::new(
                    "verification_mismatch",
                    "required milestone check failed",
                ))
            }
        }
        Action::Serve { variant, config } => {
            let server = endpoint::spawn_fixture_server(
                fixture::select_endpoint(&bundle, &variant)?,
                config,
            )?;
            emit(
                &serde_json::json!({"event":"server_started", "synthetic":true, "variant_id":variant,"url":format!("http://{}/metrics",server.bound_addr())}),
            )?;
            match server.wait() {
                Ok(outcome) => emit(
                    &serde_json::json!({"event":"server_stopped","synthetic":true,"outcome":outcome}),
                ),
                Err(e) => {
                    emit(
                        &serde_json::json!({"event":"server_stopped","synthetic":true,"error":e.to_string()}),
                    )?;
                    Err(e)
                }
            }
        }
    }
}
fn main() {
    if let Err(e) = run() {
        eprintln!(
            "{}",
            e.to_string()
                .replace(aether_poc::observation::MARKER, "[REDACTED]")
        );
        std::process::exit(if e.code == "usage" { 2 } else { 1 });
    }
}
