use crate::solver::data_model::PartialAssignment;
use crate::solver::engines::branching::Branching;
use dimacs::{Clause, Lit};
use log::trace;

pub struct VsidsBranching {
    scores: Vec<f64>,
    decay: f64,
}

impl VsidsBranching {
    pub fn new(decay: f64) -> VsidsBranching {
        VsidsBranching {
            decay,
            scores: vec![],
        }
    }
}

impl Branching for VsidsBranching {
    fn reset(&mut self, n_vars: u64) {
        self.scores = vec![0.; n_vars as usize];
    }

    fn add_clause(&mut self, clause: &Clause) {
        for lit in clause.lits() {
            self.scores[lit.var().to_u64() as usize - 1] += 1.0;
        }
    }

    fn branch(&mut self, state: &PartialAssignment) -> Option<Lit> {
        let max_score_lit = state
            .free_variables()
            .max_by(|&a, &b| self.scores[a].total_cmp(&self.scores[b]))
            .map(|ix| Lit::from_i64(-(ix as i64 + 1)));
        trace!("Branching on {:?}", max_score_lit);
        for score in self.scores.iter_mut() {
            *score *= self.decay;
        }
        max_score_lit
    }
}
