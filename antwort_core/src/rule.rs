use crate::ast::Expr;

pub enum RuleApplicationError {
    RuleNotApplicable,
}

pub struct Rule {
    pub name: String,
    pub apply: fn(&Expr) -> Result<Expr, RuleApplicationError>,
}
