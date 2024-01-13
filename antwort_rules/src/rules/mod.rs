use crate::_RULES_DISTRIBUTED_SLICE;
use antwort_core::{ast::Expr, rule::RuleApplicationError};
use antwort_macros::register_rule;

#[register_rule]
fn example_rule(_expr: &Expr) -> Result<Expr, RuleApplicationError> {
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
fn de_morgans1(expr: &Expr) -> Result<Expr, RuleApplicationError> {
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
fn de_morgans2(expr: &Expr) -> Result<Expr, RuleApplicationError> {
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

#[register_rule]
fn double_negation(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Negation(b) = expr {
        if let Expr::Negation(expr) = b.as_ref() {
            return Ok(expr.as_ref().clone());
        }
    }
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
fn implication_elimination(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Implication(a, b) = expr {
        return Ok(Expr::Disjunction(vec![
            Expr::Negation(Box::new(a.as_ref().clone())),
            b.as_ref().clone(),
        ]));
    }
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
fn flatten_conjunctions(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Conjunction(exprs) = expr {
        if exprs.iter().all(|e| !e.is_conjunction()) {
            return Err(RuleApplicationError::RuleNotApplicable);
        }
        let mut new_exprs = vec![];
        for e in exprs {
            if let Expr::Conjunction(exprs) = e {
                new_exprs.extend(exprs.iter().cloned());
            } else {
                new_exprs.push(e.clone());
            }
        }
        return Ok(Expr::Conjunction(new_exprs));
    }
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
fn flatten_disjunctions(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Disjunction(exprs) = expr {
        if exprs.iter().all(|e| !e.is_disjunction()) {
            return Err(RuleApplicationError::RuleNotApplicable);
        }
        let mut new_exprs = vec![];
        for e in exprs {
            if let Expr::Disjunction(exprs) = e {
                new_exprs.extend(exprs.iter().cloned());
            } else {
                new_exprs.push(e.clone());
            }
        }
        return Ok(Expr::Disjunction(new_exprs));
    }
    Err(RuleApplicationError::RuleNotApplicable)
}
