use std::fmt::{Display, Formatter, Write};

use dimacs::Sign;
use log::trace;

use crate::solver::data_model::Solution::*;

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

#[derive(Debug)]
pub struct PartialAssignment {
    assignment: Vec<Option<bool>>,
}

pub enum EvalResult {
    True,
    False(dimacs::Clause),
    Undefined(dimacs::Clause),
}

impl PartialAssignment {
    pub fn new(n_vars: usize) -> PartialAssignment {
        PartialAssignment {
            assignment: vec![None; n_vars],
        }
    }

    pub fn get(&self, lit: &dimacs::Lit) -> Option<bool> {
        self.assignment[(&lit.var().to_u64() - 1) as usize].map(|variable_val| match lit.sign() {
            Sign::Pos => variable_val,
            Sign::Neg => !variable_val,
        })
    }

    pub fn set(&mut self, lit: &dimacs::Lit) {
        self.assignment[(&lit.var().to_u64() - 1) as usize] = Some(lit.sign() == Sign::Pos);
    }

    pub fn unset(&mut self, lit: &dimacs::Lit) {
        self.assignment[(&lit.var().to_u64() - 1) as usize] = None;
    }

    pub fn variables(&self) -> impl Iterator<Item = usize> {
        0..self.assignment.len()
    }

    pub fn free_variables(&self) -> impl Iterator<Item = usize> + '_ {
        self.variables().filter(|&ix| self.assignment[ix].is_none())
    }

    pub fn eval(&self, clauses: &Vec<dimacs::Clause>) -> EvalResult {
        let mut eval_result = EvalResult::True;
        for clause in clauses {
            let mut clause_res = Some(false);
            for lit in clause.lits().iter() {
                match self.get(lit) {
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
                    let should_overwrite = matches!(eval_result, EvalResult::True);
                    if should_overwrite {
                        trace!(
                            "Assignment {:?} is undefined due to clause {:?}",
                            self,
                            clause
                        );
                        eval_result = EvalResult::Undefined(clause.clone());
                    }
                }
                Some(false) => {
                    trace!("Assignment {:?} is falsified by clause {:?}", self, clause);
                    eval_result = EvalResult::False(clause.clone());
                    break;
                }
                Some(true) => {}
            }
        }
        eval_result
    }

    pub fn complete(&self) -> Assignment {
        Assignment {
            assignment: self
                .assignment
                .iter()
                .map(|val| val.unwrap_or(false))
                .collect::<Vec<bool>>(),
        }
    }
}
