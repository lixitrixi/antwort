#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Variable(String),
    Negation(Box<Expr>),
    Disjunction(Vec<Expr>),
    Conjunction(Vec<Expr>),
    Implication(Box<Expr>, Box<Expr>),
    Equivalence(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Returns a vector of references to the sub-expressions of the expression.
    pub fn sub_expressions(&self) -> Vec<&Expr> {
        match self {
            Expr::Variable(_) => vec![],
            Expr::Negation(a) => vec![a.as_ref()],
            Expr::Disjunction(a) => a.iter().collect(),
            Expr::Conjunction(a) => a.iter().collect(),
            Expr::Implication(a, b) => vec![a.as_ref(), b.as_ref()],
            Expr::Equivalence(a, b) => vec![a.as_ref(), b.as_ref()],
        }
    }

    /// Creates a clone of the expression with the sub-expressions replaced by the given vector.
    pub fn with_sub_expressions(&self, sub: Vec<&Expr>) -> Expr {
        match self {
            Expr::Variable(_) => self.clone(),
            Expr::Negation(_) => Expr::Negation(Box::new(sub[0].clone())),
            Expr::Disjunction(_) => Expr::Disjunction(sub.iter().map(|e| (*e).clone()).collect()),
            Expr::Conjunction(_) => Expr::Conjunction(sub.iter().map(|e| (*e).clone()).collect()),
            Expr::Implication(_, _) => {
                Expr::Implication(Box::new(sub[0].clone()), Box::new(sub[1].clone()))
            }
            Expr::Equivalence(_, _) => {
                Expr::Equivalence(Box::new(sub[0].clone()), Box::new(sub[1].clone()))
            }
        }
    }
}
