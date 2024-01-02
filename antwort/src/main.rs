use antwort::macros::register_rule;
use antwort::rule::RuleApplicationError;
use antwort::rule_engine::{get_rules, rewrite};
use antwort::solver::{solve, Clause, Formula};
use antwort::Expr;

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

fn main() {
    let rules = get_rules();
    println!("Rules: {:?}", rules);

    let expr = Expr::Negation(Box::new(Expr::Disjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ])));
    println!("Original: {}", serde_json::to_string_pretty(&expr).unwrap());
    let res = rewrite(&expr);
    println!("Rewritten: {}", serde_json::to_string_pretty(&res).unwrap());

    let mut f = Formula::new();
    let mut c1 = Clause::new();
    c1.add_literal(-1);
    f.add_clause(c1);
    let mut c2 = Clause::new();
    c2.add_literal(1);
    c2.add_literal(2);
    f.add_clause(c2);
    let s = solve(&f);
    println!("{:?}", s);
}
