use antwort::ast::Expr;

fn main() {
    let expr = Expr::Literal("a".to_string());
    println!("{:?}", expr);
}
