use convert_case::{Case, Casing};
use std::{env::var, error::Error, fs::read_dir, fs::write, fs::File, io::Write};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = var("OUT_DIR")?;
    let dest_path = format!("{}/gen_tests.rs", out_dir);
    let mut file = File::create(&dest_path)?;

    let test_dir = "tests/integration";

    for entry in WalkDir::new(test_dir) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            let input_bases = read_dir(entry.path())?
                .filter_map(Result::ok)
                .filter(|ent| {
                    ent.file_name()
                        .into_string()
                        .unwrap()
                        .ends_with(".input.json")
                })
                .filter_map(|ent| {
                    ent.path()
                        .file_name()
                        .and_then(|stem| stem.to_str())
                        .map(|s| s.to_owned().split('.').next().unwrap().to_owned())
                })
                .collect();
            write_tests(&mut file, entry.path().to_str().unwrap(), input_bases);
        }
    }

    Ok(())
}

fn write_tests(file: &mut File, dir_path: &str, input_bases: Vec<String>) {
    for base in input_bases {
        let test_name = format!("{}_{}", dir_path, base);
        let test_name = string_to_fn_name(&test_name);
        write!(
            file,
            include_str!("./tests/gen_test_template"),
            test_name = test_name,
            dir_path = dir_path,
            input_base = base
        )
        .unwrap();
    }
}

fn string_to_fn_name(s: &str) -> String {
    s.to_case(Case::Snake)
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}
