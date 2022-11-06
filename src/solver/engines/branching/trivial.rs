use crate::solver::data_model::PartialAssignment;
use crate::solver::engines::branching::Branching;
use dimacs::{Clause, Lit};

pub struct TrivialBranching {}

impl TrivialBranching {
    pub fn new() -> Self {
        TrivialBranching {}
    }
}

impl Branching for TrivialBranching {
    fn reset(&mut self, _n_vars: u64) {}

    fn add_clause(&mut self, _clause: &Clause) {}

    fn branch(&mut self, state: &PartialAssignment) -> Option<Lit> {
        state
            .free_variables()
            .next()
            .map(|ix| Lit::from_i64(-(ix as i64 + 1)))
    }
}
