use crate::rule::Rule;
use crate::Expr;
use linkme::distributed_slice;

#[distributed_slice]
pub static RULES_DISTRIBUTED_SLICE: [Rule];

pub fn get_rules() -> Vec<Rule> {
    RULES_DISTRIBUTED_SLICE.to_vec()
}

/// Applies the first applicable rule to the expression and returns the rewritten result.
fn apply_first(expr: &Expr, rules: &Vec<Rule>) -> Option<Expr> {
    for rule in rules {
        match rule.apply(&expr) {
            Ok(new) => {
                return Some(new);
            }
            Err(_) => continue,
        }
    }
    return None;
}

/// # Returns
/// - Some(<new_expression>) after applying the first applicable rule to `expr` or a sub-expression.
/// - None if no rule is applicable to the expression or any sub-expression.
fn rewrite_iteration(expr: &Expr, rules: &Vec<Rule>) -> Option<Expr> {
    if let Some(new) = apply_first(expr, rules) {
        return Some(new);
    } else {
        let mut sub = expr.sub_expressions();
        for i in 0..sub.len() {
            if let Some(new) = rewrite_iteration(sub[i], rules) {
                sub[i] = &new;
                return Some(expr.with_sub_expressions(sub));
            }
        }
    }
    None // No rules applicable to this branch of the expression
}

/// Rewrites the expression using the rules in the registry.
/// Continues until no more rules are applicable to the expression or any sub-expression.
pub fn rewrite(expr: &Expr) -> Expr {
    println!("REWRITE: {:?}", expr);
    let rules = get_rules();
    let mut new = expr.clone();
    while let Some(step) = rewrite_iteration(&new, &rules) {
        new = step;
        println!("   STEP: {:?}", new);
    }
    println!("DONE");
    new
}
