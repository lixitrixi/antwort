use antwort::rewrite::rewrite;
use antwort::solver::{solve, Clause, Formula};

extern crate antwort_rules;

fn main() {
    use antwort::Expr;

    let expr = Expr::Equivalence(
        Box::new(Expr::Negation(Box::new(Expr::Variable("a".to_string())))),
        Box::new(Expr::Disjunction(vec![
            Expr::Variable("a".to_string()),
            Expr::Variable("b".to_string()),
        ])),
    );
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
