use antwort::rule_engine::rewrite;
use serde_json::from_str;
use std::error::Error;
use std::fs::read_to_string;

include!("../src/_rules/index.rs");

fn integration_test(dir_path: &str, input_base: &str) -> Result<(), Box<dyn Error>> {
    use antwort::Expr;

    let input_json_path = format!("{}/{}.input.json", dir_path, input_base);
    let input_str = read_to_string(&input_json_path)?;
    let input_expr: Expr = from_str(&input_str)?;

    assert!(
        rewrite(&input_expr).is_cnf(),
        "The input expression does not reduce to CNF: {}",
        input_json_path,
    );

    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/gen_tests.rs"));
