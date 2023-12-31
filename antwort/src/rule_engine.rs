use crate::rule::{Rule, RuleApplicationError};
use crate::Expr;
use linkme::distributed_slice;

#[distributed_slice]
pub static RULES: [Rule];

fn example_rule(_expr: &Expr) -> Result<Expr, RuleApplicationError> {
    Err(RuleApplicationError::RuleNotApplicable)
}
#[distributed_slice(RULES)]
static _GEN_RULE_EXAMPLE_RULE: Rule = Rule {
    application: example_rule,
};

pub fn get_rules() -> Vec<Rule> {
    RULES.to_vec()
}
