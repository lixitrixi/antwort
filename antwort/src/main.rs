use antwort::ast::Expr;
use antwort::solver::{solve, Clause, Formula};

fn main() {
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
