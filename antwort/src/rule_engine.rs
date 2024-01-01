use crate::rule::Rule;
use linkme::distributed_slice;

#[distributed_slice]
pub static RULES_DISTRIBUTED_SLICE: [Rule];

pub fn get_rules() -> Vec<Rule> {
    RULES_DISTRIBUTED_SLICE.to_vec()
}
