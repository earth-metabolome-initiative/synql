//! Test to verify that `clippy::struct_field_names` is only allowed when all
//! generated struct fields share a common prefix or suffix.

use sql_traits::prelude::ParserDB;
use synql::prelude::*;

#[test]
fn test_struct_field_names_allow() -> Result<(), Box<dyn std::error::Error>> {
    let db = ParserDB::try_from(
        r#"
        CREATE TABLE prefixed (
            obs_id INT PRIMARY KEY,
            obs_type INT,
            obs_value INT
        );

        CREATE TABLE suffixed (
            id_obs INT PRIMARY KEY,
            type_obs INT,
            value_obs INT
        );

        CREATE TABLE mixed (
            id INT PRIMARY KEY,
            foo_value INT,
            bar_kind INT
        );
    "#,
    )?;

    let temp_dir = tempfile::tempdir()?;
    let workspace_path = temp_dir.path().join("synql_struct_field_names");

    let synql: SynQL<ParserDB> = SynQL::new(&db, &workspace_path)
        .name("synql-struct-field-names")
        .generate_workspace_toml()
        .generate_rustfmt()
        .into();

    synql.generate()?;

    let prefixed_rs =
        workspace_path.join("synql-struct-field-names-prefixed").join("src").join("lib.rs");
    let suffixed_rs =
        workspace_path.join("synql-struct-field-names-suffixed").join("src").join("lib.rs");
    let mixed_rs = workspace_path.join("synql-struct-field-names-mixed").join("src").join("lib.rs");

    let prefixed = std::fs::read_to_string(prefixed_rs)?;
    let suffixed = std::fs::read_to_string(suffixed_rs)?;
    let mixed = std::fs::read_to_string(mixed_rs)?;

    eprintln!("prefixed:\n{prefixed}");
    eprintln!("suffixed:\n{suffixed}");
    eprintln!("mixed:\n{mixed}");

    assert!(prefixed.contains("# [allow (clippy :: struct_field_names)]"));
    assert!(suffixed.contains("# [allow (clippy :: struct_field_names)]"));
    assert!(!mixed.contains("# [allow (clippy :: struct_field_names)]"));

    Ok(())
}
