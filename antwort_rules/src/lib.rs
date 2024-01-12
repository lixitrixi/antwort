use antwort_core::rule::Rule;
use linkme::distributed_slice;

#[distributed_slice]
pub static _RULES_DISTRIBUTED_SLICE: [Rule];

pub fn get_rules() -> Vec<Rule> {
    _RULES_DISTRIBUTED_SLICE.to_vec()
}

pub mod rules;

static _TEMP: &Rule = &rules::_ANTWORT_GEN_RULE_EXAMPLE_RULE; // Temporary hack to force the static to be included in the binary
