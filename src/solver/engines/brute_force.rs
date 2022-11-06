use crate::solver::cnf_ops::negate_literal;
use dimacs::Clause;
use log::debug;

use crate::solver::data_model::Solution::*;
use crate::solver::data_model::*;
use crate::solver::engines::branching::Branching;
use crate::solver::SatSolver;

pub struct BruteForceSolver<B>
where
    B: Branching,
{
    assignment: PartialAssignment,
    n_vars: u64,
    brancher: B,
}

impl<B: Branching> BruteForceSolver<B> {
    pub fn new(brancher: B) -> BruteForceSolver<B> {
        BruteForceSolver {
            assignment: PartialAssignment::new(0),
            n_vars: 0,
            brancher,
        }
    }

    fn solve_from_assignment(&mut self, clauses: &Vec<Clause>) -> Solution {
        let depth = self.assignment.variables().count() - self.assignment.free_variables().count();
        self.brancher.reset(self.n_vars);
        self.brancher.add_clauses(clauses);
        match self.assignment.eval(clauses) {
            EvalResult::True => {
                debug!(
                    "[depth = {}] Satisfying assignment found: {:?}",
                    depth, self.assignment
                );
                Sat(self.assignment.complete())
            }
            EvalResult::False(_) => {
                debug!(
                    "[depth = {}] Assignment {:?} is unsatisfiable",
                    depth, self.assignment
                );
                Unsat
            }
            EvalResult::Undefined(_) => {
                let lit = self.brancher.branch(&self.assignment).unwrap();
                let neg_lit = negate_literal(&lit);
                self.assignment.set(&lit);
                if let Sat(res) = self.solve_from_assignment(clauses) {
                    return Sat(res);
                }
                self.assignment.set(&neg_lit);
                if let Sat(res) = self.solve_from_assignment(clauses) {
                    return Sat(res);
                }
                self.assignment.unset(&lit);
                debug!(
                    "[depth = {}] Assignment {:?} is unsatisfiable by exhaustion over variable {:?}",
                    depth, self.assignment, lit
                );
                Unsat
            }
        }
    }
}

impl<B> SatSolver for BruteForceSolver<B>
where
    B: Branching,
{
    fn solve(&mut self, n_vars: u64, clauses: &Vec<Clause>) -> Solution {
        self.assignment = PartialAssignment::new(n_vars as usize);
        self.n_vars = n_vars;
        self.solve_from_assignment(clauses)
    }
}
