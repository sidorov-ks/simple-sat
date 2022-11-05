use std::fs;
use std::fs::read_to_string;

use anyhow::{anyhow, bail, Context};
use clap::Parser;
use dimacs::Instance;
use log::{error, info, warn};

use crate::args::Args;
use crate::solver::engines::brute_force::BruteForceSolver;
use crate::solver::SatSolver;

mod args;
mod solver;

fn main() {
    let start = std::time::Instant::now();
    let args = Args::parse();
    init_logging();
    info!("Launch configuration {:?}", args);
    if let Err(err) = run(args) {
        error!("Solver has encountered a fatal error, details: {}", err)
    }
    info!("Shutting down after {} seconds", start.elapsed().as_secs());
}

fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

const FILENAME_PLACEHOLDER: &str = "<unknown>";

fn run(args: Args) -> anyhow::Result<()> {
    let mut solver = args.make_solver();
    let file_path = args.input_file.to_str().unwrap_or(FILENAME_PLACEHOLDER);
    let instance_text = read_to_string(&args.input_file)
        .context(format!("Failed to read the instance {}", file_path))?;
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
            let solution = solver.solve(num_vars, &clauses.into_vec());
            if fs::write(&args.output_file, format!("{}", solution)).is_err() {
                warn!(
                    "Failed to write the solution into the file {}, writing to stdout",
                    args.output_file.to_str().unwrap_or(FILENAME_PLACEHOLDER)
                );
                println!("{}", solution)
            }
            Ok(())
        }
        Instance::Sat { .. } => bail!("simple-solver does not support .sat instances"),
    }
}
