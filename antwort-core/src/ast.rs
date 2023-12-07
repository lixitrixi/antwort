#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(String),
    Negation(Box<Expr>),
    Disjunction(Vec<Expr>),
    Conjunction(Vec<Expr>),
    Implication(Box<Expr>, Box<Expr>),
    Equivalence(Box<Expr>, Box<Expr>),
}
