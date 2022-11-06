use crate::solver::data_model::PartialAssignment;
use dimacs::{Clause, Lit};

pub mod trivial;
pub mod vsids;

pub trait Branching {
    fn reset(&mut self, n_vars: u64);
    fn add_clause(&mut self, clause: &Clause);
    fn branch(&mut self, state: &PartialAssignment) -> Option<Lit>;

    fn add_clauses(&mut self, clauses: &[Clause]) {
        for clause in clauses {
            self.add_clause(clause);
        }
    }
}
