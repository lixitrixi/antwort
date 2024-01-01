use crate::ast::Expr;

#[derive(Debug)]
pub enum RuleApplicationError {
    RuleNotApplicable,
}

pub type RuleApplicationResult = Result<Expr, RuleApplicationError>;

#[derive(Clone, Debug)]
pub struct Rule {
    pub name: &'static str,
    pub application: fn(&Expr) -> RuleApplicationResult,
}

impl Rule {
    pub fn apply(&self, expr: &Expr) -> RuleApplicationResult {
        (self.application)(expr)
    }
}
