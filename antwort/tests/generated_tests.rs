use antwort::rule_engine::rewrite;
use antwort::Expr;
use serde_json::from_str;
use std::error::Error;
use std::fs::read_to_string;

fn integration_test(dir_path: &str, input_base: &str) -> Result<(), Box<dyn Error>> {
    let input_json_path = format!("{}/{}.input.json", dir_path, input_base);
    let input_str = read_to_string(&input_json_path)?;
    let input_expr: Expr = from_str(&input_str)?;

    let input_cnf_path = format!("{}/{}.input.cnf.json", dir_path, input_base);
    let input_cnf_str = read_to_string(input_cnf_path)?;
    let input_cnf_expr: Expr = from_str(&input_cnf_str)?;
    assert!(
        input_cnf_expr.is_cnf(),
        "The expected expression is not in CNF: {}",
        input_cnf_str
    );

    assert_eq!(
        rewrite(&input_expr),
        input_cnf_expr,
        "The input expression does not reduce to CNF: {}",
        input_json_path
    );

    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/gen_tests.rs"));
