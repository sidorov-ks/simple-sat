use crate::solver::data_model::Solution;

pub mod cnf_ops;
pub mod data_model;
pub mod engines;

pub trait SatSolver {
    fn solve(&mut self, n_vars: u64, clauses: &Vec<dimacs::Clause>) -> Solution;
}
