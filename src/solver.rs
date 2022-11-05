use crate::solver::Solution::*;
use dimacs::Sign;
use log::{debug, info, trace};
use std::env::var;
use std::fmt::{Display, Formatter, Write};

pub struct Assignment {
    assignment: Vec<bool>,
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, &val) in self.assignment.iter().enumerate() {
            f.write_str(format!("{}{} ", if val { "" } else { "-" }, i + 1).as_str())?;
        }
        f.write_char('0')?;
        Ok(())
    }
}

pub enum Solution {
    Sat(Assignment),
    Unsat,
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sat(x) => f.write_str(format!("SAT\n{}", x).as_str()),
            Unsat => f.write_str("UNSAT"),
        }
    }
}

pub fn solve_instance(n_vars: u64, clauses: Vec<dimacs::Clause>) -> Solution {
    let mut assignment = PartialAssignment::new(n_vars as usize);
    brute_force(&mut assignment, &clauses)
}

fn brute_force(assignment: &mut PartialAssignment, clauses: &Vec<dimacs::Clause>) -> Solution {
    let depth = assignment.variables().count() - assignment.free_variables().count();
    match assignment.eval(clauses) {
        Some(true) => {
            debug!(
                "[depth = {}] Satisfying assignment found: {:?}",
                depth, assignment
            );
            Sat(assignment.complete())
        }
        Some(false) => {
            debug!(
                "[depth = {}] Assignment {:?} is unsatisfiable",
                depth, assignment
            );
            Unsat
        }
        None => {
            let variable = assignment.free_variables().next().unwrap();
            assignment.set(variable, false);
            if let Sat(res) = brute_force(assignment, clauses) {
                return Sat(res);
            }
            assignment.set(variable, true);
            if let Sat(res) = brute_force(assignment, clauses) {
                return Sat(res);
            }
            assignment.unset(variable);
            debug!(
                "[depth = {}] Assignment {:?} is unsatisfiable by exhaustion over variable {}",
                depth, assignment, variable
            );
            Unsat
        }
    }
}

#[derive(Debug)]
struct PartialAssignment {
    assignment: Vec<Option<bool>>,
}

impl PartialAssignment {
    fn new(n_vars: usize) -> PartialAssignment {
        PartialAssignment {
            assignment: vec![None; n_vars],
        }
    }

    fn get_by_literal(&self, lit: &dimacs::Lit) -> Option<bool> {
        self.assignment[(&lit.var().to_u64() - 1) as usize].map(|variable_val| match lit.sign() {
            Sign::Pos => variable_val,
            Sign::Neg => !variable_val,
        })
    }

    fn variables(&self) -> impl Iterator<Item = usize> {
        0..self.assignment.len()
    }

    fn free_variables(&self) -> impl Iterator<Item = usize> + '_ {
        self.variables().filter(|&ix| self.assignment[ix].is_none())
    }

    fn set(&mut self, ix: usize, value: bool) {
        self.assignment[ix] = Some(value);
    }

    fn unset(&mut self, ix: usize) {
        self.assignment[ix] = None;
    }

    fn eval(&self, clauses: &Vec<dimacs::Clause>) -> Option<bool> {
        for clause in clauses {
            let mut clause_res = Some(false);
            for lit in clause.lits().iter() {
                match self.get_by_literal(lit) {
                    None => clause_res = None,
                    Some(true) => {
                        clause_res = Some(true);
                        break;
                    }
                    Some(false) => {}
                }
            }
            match clause_res {
                None => {
                    trace!(
                        "Assignment {:?} is undefined due to clause {:?}",
                        self,
                        clause
                    );
                    return None;
                }
                Some(false) => {
                    trace!("Assignment {:?} is falsified by clause {:?}", self, clause);
                    return Some(false);
                }
                Some(true) => {}
            }
        }
        Some(true)
    }

    fn complete(&self) -> Assignment {
        Assignment {
            assignment: self
                .assignment
                .iter()
                .map(|val| val.unwrap_or(false))
                .collect::<Vec<bool>>(),
        }
    }
}
