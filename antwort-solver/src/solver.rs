use crate::Formula;
use crate::{Error, Literal, Result};

#[derive(Debug, PartialEq)]
pub struct Solution {
    /// All literals that must be true for the formula to be satisfied.
    pub assignments: Vec<Literal>,
}

/// Solves the given formula using the DPLL algorithm.
pub fn solve(formula: &Formula) -> Result<Solution> {
    let mut formula = formula.clone();
    let assignments = solve_impl(&mut formula)?;
    Ok(Solution { assignments })
}

fn solve_impl(formula: &mut Formula) -> Result<Vec<Literal>> {
    if formula.is_empty() {
        return Ok(Vec::new());
    }
    if formula.contains_empty_clause() {
        return Err(Error::Unsatisfiable);
    }
    match formula.get_unit_clause() {
        Some((index, clause)) => {
            // unit clause found, literal must be true
            let literal = clause.get_literal().ok_or(Error::Unsatisfiable)?;
            formula.simplify(literal);
            let mut assignments = solve_impl(formula)?;
            assignments.push(literal);
            return Ok(assignments);
        }
        None => {
            // choose arbitrary literal and try true/false
            let literal = formula[0].get_literal().ok_or(Error::Unsatisfiable)?;
            let mut copy = formula.clone();
            formula.simplify(literal);
            match solve_impl(formula) {
                Ok(mut assignments) => {
                    assignments.push(literal);
                    return Ok(assignments);
                }
                Err(_) => {
                    copy.simplify(-literal);
                    return solve_impl(&mut copy);
                }
            }
        }
    }
}

// bool DPLL(ClauseSet* this) {
//     if (isEmptyClauseSet(this)) {
//         return true;
//     } else if (containsEmptyClause(this)) {
//         return false;
//     } else {
//         int lit;
//         int uc = findUnitClause(this);
//         if (uc >= 0) { // unit clause found, literal must be true
//             lit = getLiteral(getClause(this, uc), 0); // get the lone literal
//             simplifySet(this, lit);
//             return DPLL(this);
//         } else { // choose arbitrary literal and try true/false
//             lit = getLiteral(getClause(this, 0), 0);
//             ClauseSet copy = *this; // see report for more about this
//             simplifySet(this, lit); // try true
//             if (DPLL(this)) {
//                 return true;
//             } else {
//                 simplifySet(&copy, -lit); // try false
//                 return DPLL(&copy);
//             }
//         }
//     }
// }
