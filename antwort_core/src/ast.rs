use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

    pub fn is_variable(&self) -> bool {
        match self {
            Expr::Variable(_) => true,
            _ => false,
        }
    }

    pub fn is_negation(&self) -> bool {
        match self {
            Expr::Negation(_) => true,
            _ => false,
        }
    }

    pub fn is_disjunction(&self) -> bool {
        match self {
            Expr::Disjunction(_) => true,
            _ => false,
        }
    }

    pub fn is_conjunction(&self) -> bool {
        match self {
            Expr::Conjunction(_) => true,
            _ => false,
        }
    }

    pub fn is_implication(&self) -> bool {
        match self {
            Expr::Implication(_, _) => true,
            _ => false,
        }
    }

    pub fn is_equivalence(&self) -> bool {
        match self {
            Expr::Equivalence(_, _) => true,
            _ => false,
        }
    }

    /// Returns true if the expression is a literal.
    /// A literal is either a variable or a negated variable.
    pub fn is_literal(&self) -> bool {
        match self {
            Expr::Variable(_) => true,
            Expr::Negation(a) => a.as_ref().is_variable(),
            _ => false,
        }
    }

    /// Returns true if the expression is a clause.
    /// A clause is a disjunction of literals.
    pub fn is_clause(&self) -> bool {
        match self {
            Expr::Disjunction(a) => a.iter().all(|e| e.is_literal()),
            _ => false,
        }
    }

    /// Returns true if the expression is in conjunctive normal form.
    /// An expression is in conjunctive normal form if it is a conjunction of clauses.
    pub fn is_cnf(&self) -> bool {
        match self {
            Expr::Conjunction(a) => a.iter().all(|e| e.is_clause()),
            _ => false,
        }
    }
}
