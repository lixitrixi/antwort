use antwort::ast::Expr;

fn main() {
    let expr = Expr::Variable("a".to_string());
    println!("{:?}", expr);
}
