use crate::_RULES_DISTRIBUTED_SLICE;
use antwort_core::{ast::Expr, rule::RuleApplicationError};
use antwort_macros::register_rule;

#[register_rule]
pub fn example_rule(_expr: &Expr) -> Result<Expr, RuleApplicationError> {
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
pub fn de_morgans1(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Negation(b) = expr {
        if let Expr::Disjunction(exprs) = b.as_ref() {
            return Ok(Expr::Conjunction(
                exprs
                    .iter()
                    .map(|e| Expr::Negation(Box::new(e.clone())))
                    .collect(),
            ));
        }
    }
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
pub fn de_morgans2(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Negation(b) = expr {
        if let Expr::Conjunction(exprs) = b.as_ref() {
            return Ok(Expr::Disjunction(
                exprs
                    .iter()
                    .map(|e| Expr::Negation(Box::new(e.clone())))
                    .collect(),
            ));
        }
    }
    Err(RuleApplicationError::RuleNotApplicable)
}
