//! Test module to verify the callback functionality in SynQL.

use quote::quote;
use sql_traits::prelude::ParserDB;
use sqlparser::dialect::GenericDialect;
use synql::prelude::*;

#[test]
fn test_callback_generation() -> Result<(), Box<dyn std::error::Error>> {
    let db = ParserDB::parse::<GenericDialect>("CREATE TABLE users (id SERIAL PRIMARY KEY);")?;
    let temp_dir = tempfile::tempdir()?;
    let workspace_path = temp_dir.path().join("synql_workspace");

    let synql: SynQL<ParserDB> = SynQL::new(&db, &workspace_path)
        .callback(|_table, _db, _workspace| {
            Ok(Some(quote! {
                pub fn hello_world() -> &'static str {
                    "Hello, World!"
                }
            }))
        })
        .toml_callback(|_table, _db| Ok(Some(TomlDependency::new("extra-dep").version("1.0.0")?)))
        .into();

    synql.generate()?;

    let mut lib_rs_path = workspace_path.join("unknown_catalog-users").join("src").join("lib.rs");

    if !lib_rs_path.exists() {
        // Fallback: look for directory ending with users
        for entry in std::fs::read_dir(&workspace_path)? {
            let path = entry?.path();
            if path.is_dir()
                && path.file_name().unwrap_or_default().to_string_lossy().ends_with("users")
            {
                lib_rs_path = path.join("src").join("lib.rs");
                break;
            }
        }
    }

    let content = std::fs::read_to_string(lib_rs_path.clone())?;
    assert!(content.contains("pub fn hello_world"));

    let cargo_toml_path = lib_rs_path.parent().unwrap().parent().unwrap().join("Cargo.toml");
    let content = std::fs::read_to_string(cargo_toml_path)?;
    assert!(content.contains("extra-dep = { version = \"1.0.0\" }"));

    Ok(())
}
