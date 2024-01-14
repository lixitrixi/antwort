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

#[register_rule]
fn equicalence_elimination(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Equivalence(a, b) = expr {
        return Ok(Expr::Conjunction(vec![
            Expr::Implication(a.clone(), b.clone()),
            Expr::Implication(b.clone(), a.clone()),
        ]));
    }
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
fn distribute_negation_disjunction(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Negation(b) = expr {
        if let Expr::Disjunction(exprs) = b.as_ref() {
            let mut new_exprs = vec![];
            for e in exprs {
                new_exprs.push(Expr::Negation(Box::new(e.clone())));
            }
            return Ok(Expr::Conjunction(vec![Expr::Disjunction(new_exprs)]));
        }
    }
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
fn distribute_negation_conjunction(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Negation(b) = expr {
        if let Expr::Conjunction(exprs) = b.as_ref() {
            let mut new_exprs = vec![];
            for e in exprs {
                new_exprs.push(Expr::Negation(Box::new(e.clone())));
            }
            return Ok(Expr::Disjunction(vec![Expr::Conjunction(new_exprs)]));
        }
    }
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
fn unwrap_single_disjunctions(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Disjunction(exprs) = expr {
        if exprs.len() == 1 {
            return Ok(exprs[0].clone());
        }
    }
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
fn unwrap_single_conjunctions(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Conjunction(exprs) = expr {
        if exprs.len() == 1 {
            return Ok(exprs[0].clone());
        }
    }
    Err(RuleApplicationError::RuleNotApplicable)
}

#[register_rule]
fn distribute_disjunctions(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    if let Expr::Disjunction(exprs) = expr {
        if exprs.len() < 2 {
            return Err(RuleApplicationError::RuleNotApplicable);
        }
        let a = &exprs[0];
        if let Expr::Conjunction(a_exprs) = a {
            let b = &exprs[1];
            if let Expr::Conjunction(b_exprs) = b {
                let mut combinations = vec![];
                for a_expr in a_exprs {
                    for b_expr in b_exprs {
                        combinations.push(Expr::Disjunction(vec![a_expr.clone(), b_expr.clone()]));
                    }
                }
                let mut new = vec![Expr::Conjunction(combinations)];
                new.extend_from_slice(&exprs[2..]);
                return Ok(Expr::Disjunction(new));
            } else {
                let mut combinations = vec![];
                for a_expr in a_exprs {
                    combinations.push(Expr::Disjunction(vec![a_expr.clone(), b.clone()]));
                }
                let mut new = vec![Expr::Conjunction(combinations)];
                new.extend_from_slice(&exprs[2..]);
                return Ok(Expr::Disjunction(new));
            }
        } else {
            let b = exprs[1..]
                .iter()
                .enumerate()
                .find(|(_, e)| e.is_conjunction());
            if let Some((i, Expr::Conjunction(b_exprs))) = b {
                let mut combinations = vec![];
                for b_expr in b_exprs {
                    combinations.push(Expr::Disjunction(vec![a.clone(), b_expr.clone()]));
                }
                let mut new = vec![Expr::Conjunction(combinations)];
                for (_, e) in exprs[1..].iter().enumerate().filter(|(j, _)| *j != i) {
                    new.push(e.clone());
                }
                return Ok(Expr::Disjunction(new));
            }
        }
    }
    Err(RuleApplicationError::RuleNotApplicable)
}
