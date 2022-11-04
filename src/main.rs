mod solver;

use std::fs;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::{anyhow, bail, Context};
use dimacs::Instance;
use log::{error, info, warn};

fn main() {
    init_logging();
    let instance = std::env::args().nth(1);
    let output = std::env::args().nth(2);
    if let (Some(input_file_path), Some(output_file_path)) = (instance, output) {
        if let Err(err) = run(Path::new(&input_file_path), Path::new(&output_file_path)) {
            error!("Solver has encountered a fatal error, details: {}", err)
        }
        info!("Shutting down")
    } else {
        eprintln!("Usage: simple-sat <instance> <output>")
    }
}

fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

const FILENAME_PLACEHOLDER: &str = "<unknown>";

fn run(input_file: &Path, output_file: &Path) -> anyhow::Result<()> {
    let file_path = input_file.to_str().unwrap_or(FILENAME_PLACEHOLDER);
    let instance_text =
        read_to_string(input_file).context(format!("Failed to read the instance {}", file_path))?;
    match dimacs::parse_dimacs(&instance_text)
        .map_err(|err| anyhow!("Loc {:?}, error kind: {:?}", err.loc, err.kind))
        .context(format!("Failed to parse the instance {}", file_path))?
    {
        Instance::Cnf { num_vars, clauses } => {
            info!(
                "Solving the instance {} with {} variables and {} constraints",
                file_path,
                num_vars,
                clauses.len()
            );
            let solution = solver::solve_instance(num_vars, clauses.into_vec());
            if let Err(_) = fs::write(output_file, format!("{}", solution)) {
                warn!(
                    "Failed to write the solution into the file {}, writing to stdout",
                    output_file.to_str().unwrap_or(FILENAME_PLACEHOLDER)
                );
                println!("{}", solution)
            }
            Ok(())
        }
        Instance::Sat { .. } => bail!("simple-solver does not support .sat instances"),
    }
}
