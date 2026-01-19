//! Generates a schema into a persistent repo directory and prints inputs/outputs.
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use sql_traits::prelude::ParserDB;
use synql::prelude::*;

fn main() {
    let input_sql = "\
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE
);

CREATE TABLE species (
    id SERIAL PRIMARY KEY,
    scientific_name TEXT NOT NULL UNIQUE
);

CREATE TABLE molecules (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    formula TEXT
);

CREATE TABLE observations (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    species_id INTEGER NOT NULL REFERENCES species(id),
    observed_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE observation_molecules (
    observation_id INTEGER NOT NULL REFERENCES observations(id),
    molecule_id INTEGER NOT NULL REFERENCES molecules(id),
    concentration REAL NOT NULL,
    PRIMARY KEY (observation_id, molecule_id)
);
";
    let repo_output_dir = PathBuf::from("synql/examples/generated_schema");
    let input_dir = repo_output_dir.join("input");
    let output_dir = repo_output_dir.join("output");

    fs::create_dir_all(&input_dir).expect("Unable to create input directory");
    fs::create_dir_all(&output_dir).expect("Unable to create output directory");

    let file_path = input_dir.join("model.sql");
    let mut file = File::create(&file_path).expect("Unable to create input SQL file");
    writeln!(file, "{input_sql}").expect("Unable to write input SQL");

    println!("Input SQL file: {}", file_path.display());
    println!("Input SQL:\n{input_sql}\n");

    let db = ParserDB::try_from(input_dir.as_path()).expect("Failed to parse database schema");

    let synql: SynQL<ParserDB> =
        SynQL::new(&db, &output_dir).name("document_schema").generate_workspace_toml().into();

    synql.generate().expect("Unable to generate workspace");

    println!("Generated workspace at: {}\n", output_dir.display());
    print_rs_files(&output_dir);
}

fn print_rs_files(dir: &Path) {
    let mut rs_files = Vec::new();
    collect_rs_files(dir, &mut rs_files);
    rs_files.sort();

    if rs_files.is_empty() {
        println!("No Rust files found in the generated workspace.");
        return;
    }

    println!("Generated Rust files and contents:\n");
    for path in rs_files {
        println!("--- {} ---", path.display());
        match fs::read_to_string(&path) {
            Ok(contents) => {
                for line in contents.lines() {
                    println!("{line}");
                }
            }
            Err(err) => println!("(unable to read file: {err})"),
        }
        println!();
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}
