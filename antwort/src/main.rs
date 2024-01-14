use antwort::rewrite::rewrite;
use antwort::solver::{solve, Clause, Formula};
use antwort::Expr;
use clap::{arg, command, Parser};
use serde_json::from_str;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "INPUT_FILE")]
    input_file: PathBuf,

    #[arg(value_name = "INPUT_FORMAT", default_value_t = ("json".to_string()))]
    input_format: String,
}

fn main() {
    let cli = Cli::parse();
    let input_str = std::fs::read_to_string(cli.input_file).unwrap();
    let input: Expr = match cli.input_format.as_str() {
        "json" => from_str(&input_str).unwrap(),
        _ => panic!("Unknown input format: {}", cli.input_format),
    };

    let cnf = rewrite(&input);
    println!("Rewritten: {}", serde_json::to_string_pretty(&cnf).unwrap());
    assert!(cnf.is_cnf());
}
