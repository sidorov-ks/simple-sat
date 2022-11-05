use dimacs::Clause;
use log::debug;

use crate::solver::data_model::Solution::*;
use crate::solver::data_model::*;
use crate::solver::SatSolver;

pub struct BruteForceSolver {
    assignment: PartialAssignment,
}

impl BruteForceSolver {
    pub fn new() -> BruteForceSolver {
        BruteForceSolver {
            assignment: PartialAssignment::new(0),
        }
    }

    fn solve_from_assignment(&mut self, clauses: &Vec<Clause>) -> Solution {
        let depth = self.assignment.variables().count() - self.assignment.free_variables().count();
        match self.assignment.eval(clauses) {
            Some(true) => {
                debug!(
                    "[depth = {}] Satisfying assignment found: {:?}",
                    depth, self.assignment
                );
                Sat(self.assignment.complete())
            }
            Some(false) => {
                debug!(
                    "[depth = {}] Assignment {:?} is unsatisfiable",
                    depth, self.assignment
                );
                Unsat
            }
            None => {
                let variable = self.assignment.free_variables().next().unwrap();
                self.assignment.set(variable, false);
                if let Sat(res) = self.solve_from_assignment(clauses) {
                    return Sat(res);
                }
                self.assignment.set(variable, true);
                if let Sat(res) = self.solve_from_assignment(clauses) {
                    return Sat(res);
                }
                self.assignment.unset(variable);
                debug!(
                    "[depth = {}] Assignment {:?} is unsatisfiable by exhaustion over variable {}",
                    depth,
                    self.assignment,
                    variable + 1
                );
                Unsat
            }
        }
    }
}

impl SatSolver for BruteForceSolver {
    fn solve(&mut self, n_vars: u64, clauses: &Vec<Clause>) -> Solution {
        self.assignment = PartialAssignment::new(n_vars as usize);
        self.solve_from_assignment(clauses)
    }
}
