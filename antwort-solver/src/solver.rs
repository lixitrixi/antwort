use crate::Formula;
use crate::{Error, Literal, Result};

pub struct Solution {
    /// All literals that must be true for the formula to be satisfied.
    pub assignments: Vec<Literal>,
}

/// Solves the given formula using the DPLL algorithm.
pub fn solve(formula: &Formula) -> Result<Solution> {
    let mut formula = formula.clone();
    let mut assignments = Vec::new();
    solve_impl(&mut formula, &mut assignments)?;
    Ok(Solution { assignments })
}

fn solve_impl(formula: &mut Formula, assignments: &mut Vec<Literal>) -> Result<()> {
    todo!()
}
