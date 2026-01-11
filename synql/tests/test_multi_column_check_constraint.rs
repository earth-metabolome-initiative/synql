//! Test to verify that tables with composite multi-column check constraints
//! are generated correctly.

use sql_traits::prelude::ParserDB;
use synql::prelude::*;

#[test]
fn test_multi_column_check_constraint() -> Result<(), Box<dyn std::error::Error>> {
    let db = ParserDB::try_from(
        "
    CREATE TABLE test_table (
        id INT PRIMARY KEY,
        value1 INT,
        value2 INT,
        CHECK (value1 + value2 > 0),
        CHECK (value1 < value2)
    );
",
    )?;

    let temp_dir = tempfile::tempdir().expect("Unable to create temporary directory");
    let workspace_path = temp_dir.path().join("synql_multi_column_check");

    let synql: SynQL<ParserDB> = SynQL::new(&db, &workspace_path)
        .name("synql-multi-column-check")
        .generate_workspace_toml()
        .generate_rustfmt()
        .into();
    synql.generate().expect("Unable to generate workspace");

    let crate_name = "synql-multi-column-check-test_table";
    let rs_path = workspace_path.join(crate_name).join("src").join("lib.rs");

    let content = std::fs::read_to_string(&rs_path)
        .unwrap_or_else(|e| panic!("Could not read file at {rs_path:?}: {e}"));

    let normalized: String = content.chars().filter(|c| !c.is_whitespace()).collect();

    let expected_impl_value1 = "impl :: diesel_builders :: ValidateColumn < test_table :: value1 >                                 for < test_table :: table as :: diesel_builders :: TableExt > :: NewValues";
    let expected_impl_value1_norm: String =
        expected_impl_value1.chars().filter(|c| !c.is_whitespace()).collect();

    assert!(
        normalized.contains(&expected_impl_value1_norm),
        "Missing ValidateColumn impl for value1"
    );

    assert!(
        normalized.contains("value1+value2>0"),
        "Missing check 1 logic (sum > 0). Found:\n{content}"
    );

    assert!(
        normalized.contains("strictly_smaller_than"),
        "Missing strictly_smaller_than validation error. Found:\n{content}"
    );

    assert!(
        normalized.contains("value1>=value2"),
        "Missing negated check logic (value1 >= value2). Found:\n{content}"
    );

    Ok(())
}
