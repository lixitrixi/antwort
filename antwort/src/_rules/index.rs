use antwort::{macros::register_rule, rule::RuleApplicationError, Expr};

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
