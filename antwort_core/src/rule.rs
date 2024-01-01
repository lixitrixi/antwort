use crate::ast::Expr;

#[derive(Debug)]
pub enum RuleApplicationError {
    RuleNotApplicable,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub name: &'static str,
    pub application: fn(&Expr) -> Result<Expr, RuleApplicationError>,
}

impl Rule {
    pub fn apply(&self, expr: &Expr) -> Result<Expr, RuleApplicationError> {
        (self.application)(expr)
    }
}
