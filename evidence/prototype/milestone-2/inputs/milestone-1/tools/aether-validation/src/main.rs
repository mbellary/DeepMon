use aether_validation::{check_evidence, check_traceability, generate_records};
use std::path::PathBuf;
fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|s| s == "--help" || s == "-h") {
        println!(
            "aether-validate <evidence|traceability|generate> [--root PATH]\nLocal integrity/specification checks only; no engine or hardware access."
        );
        return std::process::ExitCode::SUCCESS;
    }
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    if args.len() == 3 && args[1] == "--root" {
        root = PathBuf::from(&args[2]);
    } else if args.len() != 1 {
        eprintln!("usage: aether-validate <evidence|traceability|generate> [--root PATH]");
        return std::process::ExitCode::from(2);
    }
    let result=match args[0].as_str() {
        "evidence"=>check_evidence(&root).map(|s|println!("PASS: {} pinned upstream inputs; {} artifact checksums; {}/26 historical U rows reconciled.\nDocumentation routes: E=88 R=35 H=100 I=26 U=1; L1-001 remains U and L1-005 inherits its semantic gap.\nDeployed schemas/capabilities: NOT VERIFIED.",s.sources,s.artifacts,s.historical_u)),
        "traceability"=>check_traceability(&root).map(|s|println!("metric_requirements={}\nmetadata_associations={}\nexperiment_specifications={}\nselected_metrics={}\nunselected_metrics={}\nselected_associations={}\nunselected_associations={}\nmissing_original_ids=0\nduplicate_ids=0\nsynthetic_experiments_executed=0\ndeployed_schemas_and_capabilities=not_verified\nruntime_acceptance_readiness=false\nmilestone_1_traceability=pass",s.metrics,s.associations,s.experiments,s.selected_metrics,s.metrics-s.selected_metrics,s.selected_associations,s.associations-s.selected_associations)),
        "generate"=>generate_records(&root).map(|n|println!("Created {n} planning files; identical existing files retained. No fixtures or runtime results generated.")),
        _=>{eprintln!("unknown command: {}",args[0]);return std::process::ExitCode::from(2);}
    };
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(violations) => {
            for e in violations {
                eprintln!("FAIL: {e}");
            }
            std::process::ExitCode::FAILURE
        }
    }
}
