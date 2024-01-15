use antwort_core::rule::Rule;
use linkme::distributed_slice;

#[distributed_slice]
pub static _RULES_DISTRIBUTED_SLICE: [Rule];

pub fn get_rules() -> Vec<Rule> {
    _RULES_DISTRIBUTED_SLICE.to_vec()
}

mod rules;

static _TEMP: &Rule = &rules::_ANTWORT_GEN_RULE_DE_MORGANS1; // Temporary hack to force the static to be included in the binary
