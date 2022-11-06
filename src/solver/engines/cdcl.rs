use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::solver::cnf_ops::{negate_literal, resolve};
use dimacs::{Clause, Lit, Sign};
use log::{debug, trace};

use crate::solver::data_model::Solution::*;
use crate::solver::data_model::*;
use crate::solver::engines::branching::Branching;
use crate::solver::SatSolver;

#[derive(Eq, PartialEq, Debug)]
struct HashLit(Lit);

impl Hash for HashLit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i64(
            self.0.var().to_u64() as i64 * if self.0.sign() == Sign::Pos { 1 } else { -1 },
        );
    }
}

pub struct CdclSolver<B>
where
    B: Branching,
{
    assignment: PartialAssignment,
    clauses: Vec<Clause>,
    implications: HashMap<HashLit, Clause>,
    current_decision_level: u64,
    lit_decision_levels: HashMap<HashLit, u64>,
    brancher: B,
}

impl<B: Branching> CdclSolver<B> {
    pub fn new(brancher: B) -> CdclSolver<B> {
        CdclSolver {
            assignment: PartialAssignment::new(0),
            clauses: vec![],
            implications: HashMap::new(),
            current_decision_level: 0,
            lit_decision_levels: HashMap::new(),
            brancher,
        }
    }

    fn unit_propagate(&mut self) {
        loop {
            if !self.unit_propagate_step() {
                break;
            }
        }
        debug!(
            "[DL = {}] Unit propagation yields assignment {:?}",
            self.current_decision_level, self.assignment
        );
    }

    fn branch(&mut self) {
        self.current_decision_level += 1;
        // let variable = self.assignment.free_variables().next().unwrap();
        // let value = false;
        // self.assignment.set(variable, value);
        let lit = self.brancher.branch(&self.assignment).unwrap();
        self.assignment.set(&lit);
        debug!(
            "[DL = {}] Setting literal {:?} to true",
            self.current_decision_level, lit
        );
        // let lit = Lit::from_i64(((variable + 1) as i64) * (if value { 1 } else { -1 }));
        self.lit_decision_levels
            .insert(HashLit(lit), self.current_decision_level);
    }

    fn analyze_conflict(&mut self, conflict_clause: Clause) -> Clause {
        let mut derived_clause = conflict_clause.clone();
        loop {
            trace!(
                "Analyzed {:?} into {:?} so far",
                conflict_clause,
                derived_clause
            );
            let mut resolved_clause = None;
            for lit in derived_clause.lits() {
                let neg_lit = negate_literal(lit);
                let is_current_level = self.lit_decision_levels.get(&HashLit(neg_lit)).cloned()
                    == Some(self.current_decision_level);
                if is_current_level && self.implications.contains_key(&HashLit(neg_lit)) {
                    let implied_clause = &self.implications[&HashLit(neg_lit)];
                    resolved_clause = Some(resolve(implied_clause, &derived_clause, lit));
                    trace!(
                        "Resolving {:?} and {:?} into {:?}",
                        derived_clause,
                        implied_clause,
                        resolved_clause.as_ref().unwrap()
                    );
                    break;
                }
            }
            match resolved_clause {
                None => break,
                Some(clause) => derived_clause = clause,
            }
        }
        trace!(
            "Completed analysis of clause {:?}, the resulting clause is {:?}",
            conflict_clause,
            derived_clause
        );
        debug!(
            "[DL = {}] Derived a new clause {:?}",
            self.current_decision_level, derived_clause
        );
        trace!(
            "[DL = {}] Decision levels of the literals: {:?}",
            self.current_decision_level,
            self.lit_decision_levels
        );
        derived_clause
    }

    fn unit_propagate_step(&mut self) -> bool {
        for clause in self.clauses.iter() {
            if let Some(lit) = self.unit_propagate_clause(clause) {
                self.assignment.set(&lit);
                self.implications.insert(HashLit(lit), clause.clone());
                self.lit_decision_levels
                    .insert(HashLit(lit), self.current_decision_level);
                return true;
            }
        }
        false
    }

    fn unit_propagate_clause(&self, clause: &Clause) -> Option<Lit> {
        let mut result = None;
        for lit in clause.lits() {
            match self.assignment.get(lit) {
                Some(true) => {
                    return None;
                }
                Some(false) => {}
                None => {
                    if result.is_some() {
                        return None;
                    }
                    result = Some(*lit);
                }
            }
        }
        result
    }

    fn backjump(&mut self, decision_level: u64) {
        self.current_decision_level = decision_level;
        let unset_lits = self
            .lit_decision_levels
            .iter()
            .filter(|&(_, &level)| level > decision_level)
            .map(|(lit, _)| lit.0)
            .collect::<Vec<Lit>>();
        for unset_lit in unset_lits {
            let hash_unset_lit = HashLit(unset_lit);
            self.lit_decision_levels.remove(&hash_unset_lit);
            self.assignment.unset(&unset_lit);
            self.implications.remove(&hash_unset_lit);
        }
        debug!(
            "[DL = {}] Backjumping to level {} to the assignment {:?}",
            self.current_decision_level, decision_level, self.assignment
        );
    }

    fn eval_decision_level(&self, learned_clause: &Clause) -> u64 {
        learned_clause
            .lits()
            .iter()
            .map(|lit| {
                let neg_lit = Lit::from_i64(
                    lit.var().to_u64() as i64 * if lit.sign() == Sign::Pos { -1 } else { 1 },
                );
                self.lit_decision_levels
                    .get(&HashLit(neg_lit))
                    .cloned()
                    .unwrap_or(0)
            })
            .filter(|&dl| dl != self.current_decision_level)
            .max()
            .unwrap_or(0)
    }

    fn reset(&mut self, n_vars: u64, clauses: &[Clause]) {
        self.assignment = PartialAssignment::new(n_vars as usize);
        self.clauses = clauses.to_vec();
        self.implications = HashMap::new();
        self.current_decision_level = 0;
        self.lit_decision_levels = HashMap::new();
        self.brancher.reset(n_vars);
        self.brancher.add_clauses(clauses);
    }
}

impl<B: Branching> SatSolver for CdclSolver<B> {
    fn solve(&mut self, n_vars: u64, clauses: &Vec<Clause>) -> Solution {
        self.reset(n_vars, clauses);
        loop {
            self.unit_propagate();
            match self.assignment.eval(clauses) {
                EvalResult::True => {
                    debug!(
                        "[DL = {}] Satisfying assignment found with {} learned clauses: {:?}",
                        self.current_decision_level,
                        self.clauses.len() - clauses.len(),
                        self.assignment
                    );
                    return Sat(self.assignment.complete());
                }
                EvalResult::False(conflict_clause) => {
                    if self.current_decision_level == 0 {
                        debug!(
                            "[DL = {}] Derived a contradiction with {} learned clauses, terminating",
                            self.current_decision_level, self.clauses.len() - clauses.len()
                        );
                        return Unsat;
                    }
                    let learned_clause = self.analyze_conflict(conflict_clause);
                    self.brancher.add_clause(&learned_clause);
                    self.clauses.push(learned_clause.clone());
                    let max_decision_level = self.eval_decision_level(&learned_clause);
                    self.backjump(max_decision_level);
                }
                EvalResult::Undefined(_) => self.branch(),
            }
        }
    }
}
