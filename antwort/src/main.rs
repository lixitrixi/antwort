use antwort::macros::register_rule;
use antwort::rule::RuleApplicationError;
use antwort::rule_engine::get_rules;
use antwort::solver::{solve, Clause, Formula};
use antwort::Expr;

#[register_rule]
fn example_rule(_expr: &Expr) -> Result<Expr, RuleApplicationError> {
    Err(RuleApplicationError::RuleNotApplicable)
}

fn main() {
    let rules = get_rules();
    println!("{:?}", rules);
    let res = rules[0].apply(&Expr::Variable("a".to_string()));
    println!("{:?}", res);

    let expr = Expr::Variable("a".to_string());
    println!("{:?}", expr);

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
