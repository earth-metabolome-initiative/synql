//! Test module to verify the callback functionality in SynQL.

use quote::quote;
use sql_traits::prelude::ParserDB;
use synql::prelude::*;

#[test]
fn test_callback_generation() -> Result<(), Box<dyn std::error::Error>> {
    let db = ParserDB::try_from("CREATE TABLE users (id SERIAL PRIMARY KEY);")?;
    let temp_dir = tempfile::tempdir()?;
    let workspace_path = temp_dir.path().join("synql_workspace");

    let synql: SynQL<ParserDB> = SynQL::new(&db, &workspace_path)
        .callback(|_table, _db| {
            Ok(quote! {
                pub fn hello_world() -> &'static str {
                    "Hello, World!"
                }
            })
        })
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

    let content = std::fs::read_to_string(lib_rs_path)?;
    assert!(content.contains("pub fn hello_world"));

    Ok(())
}
