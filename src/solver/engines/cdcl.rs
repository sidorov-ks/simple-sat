use crate::solver::data_model::Solution::*;
use crate::solver::data_model::*;
use crate::solver::SatSolver;

use dimacs::Clause;

pub struct CdclSolver {
    assignment: PartialAssignment,
}

impl CdclSolver {
    pub fn new() -> CdclSolver {
        CdclSolver {
            assignment: PartialAssignment::new(0),
        }
    }
}

impl SatSolver for CdclSolver {
    fn solve(&mut self, n_vars: u64, clauses: &Vec<Clause>) -> Solution {
        self.assignment = PartialAssignment::new(n_vars as usize);
        // TODO
        Solution::Unsat
    }
}
