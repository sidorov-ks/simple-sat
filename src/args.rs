use clap::Parser;
use std::path::PathBuf;

use crate::solver::engines::cdcl::CdclSolver;
use crate::{BruteForceSolver, SatSolver};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short)]
    pub(crate) input_file: PathBuf,

    #[arg(short)]
    pub(crate) output_file: PathBuf,

    #[arg(short, long, default_value = "cdcl")]
    solver_type: SolverType,
}

#[derive(Copy, Clone, clap::ValueEnum, Debug)]
enum SolverType {
    BruteForce,
    Cdcl,
}

impl Args {
    pub fn make_solver(&self) -> Box<dyn SatSolver> {
        match self.solver_type {
            SolverType::BruteForce => Box::new(BruteForceSolver::new()),
            SolverType::Cdcl => Box::new(CdclSolver::new()),
        }
    }
}
