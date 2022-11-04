use crate::solver::Solution::Unsat;
use std::fmt::{Display, Formatter};

pub struct Assignment {
    assignment: Vec<bool>,
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, &val) in self.assignment.iter().enumerate() {
            f.write_str(format!("{}{}", if val { "" } else { "-" }, i + 1).as_str())?;
        }
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
            Solution::Sat(x) => f.write_str(format!("SAT\n{}", x).as_str()),
            Solution::Unsat => f.write_str("UNSAT"),
        }
    }
}

pub fn solve_instance(n_vars: u64, clauses: Vec<dimacs::Clause>) -> Solution {
    Unsat
}
