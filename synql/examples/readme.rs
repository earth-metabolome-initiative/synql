//! Example module for synql
use std::fs::File;
use std::io::Write;

use sql_traits::prelude::ParserDB;
use synql::prelude::*;
use tempfile::tempdir;

fn main() {
    // Setup a temporary directory with a SQL file
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("model.sql");
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT);").unwrap();

    // Parse the directory
    // Note: ParserDB::try_from usually takes a path to a directory or file structure
    let db = ParserDB::try_from(dir.path()).expect("Failed to parse database schema");

    // Generate to a temporary output path
    let output_dir = tempdir().unwrap();

    let synql: SynQL<ParserDB> =
        SynQL::new(&db, output_dir.path()).name("document_schema").generate_workspace_toml().into();

    synql.generate().expect("Unable to generate workspace");
}
