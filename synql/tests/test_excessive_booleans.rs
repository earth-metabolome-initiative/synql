//! Test module to verify structs with `bool` fields above the set threshold are being decorated with `#[allow(clippy::struct_excessive_bools)]`.

use sql_traits::prelude::ParserDB;
use synql::prelude::*;

#[test]
fn test_excessive_booleans() -> Result<(), Box<dyn std::error::Error>> {
    let db = ParserDB::try_from(
        r"
    CREATE TABLE no_booleans (
        id INT PRIMARY KEY, 
        field_a INT
    );
    CREATE TABLE booleans (
        id INT PRIMARY KEY, 
        field_a BOOL NOT NULL,
        field_b BOOL NOT NULL,
        field_c BOOL NOT NULL,
        field_d BOOL NOT NULL,
        field_e BOOL NOT NULL
    );
    ",
    )?;

    let temp_dir = tempfile::tempdir().expect("Unable to create temporary directory");
    let workspace_path = temp_dir.path().join("excessive_booleans");

    let synql: SynQL<ParserDB> = SynQL::new(&db, &workspace_path)
        .name("excessive-booleans")
        .generate_workspace_toml()
        .generate_rustfmt()
        .into();
    synql.generate().expect("Unable to generate workspace");

    // Construct the expected path to the generated file
    // The crate name is constructed from the workspace name and table name
    let no_booleans_crate_name = "excessive-booleans-no_booleans";
    let no_bool_rs_path = workspace_path
        .join(no_booleans_crate_name)
        .join("src")
        .join("lib.rs");
    let booleans_crate_name = "excessive-booleans-booleans";
    let bool_rs_path = workspace_path
        .join(booleans_crate_name)
        .join("src")
        .join("lib.rs");

    let no_bool_content = std::fs::read_to_string(&no_bool_rs_path)
        .unwrap_or_else(|e| panic!("Could not read file at {no_bool_rs_path:?}: {e}"));

    let bool_content = std::fs::read_to_string(&bool_rs_path)
        .unwrap_or_else(|e| panic!("Could not read file at {bool_rs_path:?}: {e}"));

    // Normalize content by removing all whitespace to avoid formatting issues
    let normalized_no_bool_content: String = no_bool_content.chars().filter(|c| !c.is_whitespace()).collect();
    let normalized_bool_content: String = bool_content.chars().filter(|c| !c.is_whitespace()).collect();
    let expected_decorator = "#[allow(clippy::struct_excessive_bools)]";

    assert!(
        !normalized_no_bool_content.contains(expected_decorator),
        "No Bool struct should not have {expected_decorator} decorator. Content:\n{no_bool_content}",
    );

    assert!(
        normalized_bool_content.contains(expected_decorator),
        "Bool struct should have {expected_decorator} decorator. Content:\n{bool_content}",
    );

    Ok(())
}
