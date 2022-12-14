use clap::Parser;
use const_format::concatcp;
use std::path::PathBuf;

use crate::solver::engines::branching::trivial::TrivialBranching;
use crate::solver::engines::branching::vsids::VsidsBranching;
use crate::solver::engines::cdcl::CdclSolver;
use crate::{BruteForceSolver, SatSolver};

pub static VERSION_STRING: &str = concatcp!(
    "\n",
    "\n",
    "Build timestamp:\t",
    env!("VERGEN_BUILD_TIMESTAMP"),
    "\n",
    "Build version:  \t",
    env!("VERGEN_GIT_SEMVER"),
    "\n",
    "Commit SHA:     \t",
    env!("VERGEN_GIT_SHA"),
    "\n",
    "Commit date:    \t",
    env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
    "\n",
    "Commit branch:  \t",
    env!("VERGEN_GIT_BRANCH"),
);

#[derive(Parser, Debug)]
#[command(author, version = VERSION_STRING, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short)]
    pub(crate) input_file: PathBuf,

    #[arg(short)]
    pub(crate) output_file: PathBuf,

    #[arg(short, long, default_value = "cdcl")]
    solver_type: SolverType,

    #[arg(short, long, default_value = "vsids")]
    branching_type: BranchingType,

    #[arg(long, default_value_t = 0.95)]
    vsids_decay: f64,
}

#[derive(Copy, Clone, clap::ValueEnum, Debug)]
enum SolverType {
    BruteForce,
    Cdcl,
}

#[derive(Copy, Clone, clap::ValueEnum, Debug)]
enum BranchingType {
    Trivial,
    Vsids,
}

impl Args {
    pub fn make_solver(&self) -> Box<dyn SatSolver> {
        let trivial_branching = TrivialBranching::new();
        let vsids_branching = VsidsBranching::new(self.vsids_decay);
        match (self.solver_type, self.branching_type) {
            (SolverType::BruteForce, BranchingType::Trivial) => {
                Box::new(BruteForceSolver::<TrivialBranching>::new(trivial_branching))
            }
            (SolverType::BruteForce, BranchingType::Vsids) => {
                Box::new(BruteForceSolver::<VsidsBranching>::new(vsids_branching))
            }
            (SolverType::Cdcl, BranchingType::Trivial) => {
                Box::new(CdclSolver::<TrivialBranching>::new(trivial_branching))
            }
            (SolverType::Cdcl, BranchingType::Vsids) => {
                Box::new(CdclSolver::<VsidsBranching>::new(vsids_branching))
            }
        }
    }
}
