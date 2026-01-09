//! Submodule providing the `TableListLike` trait, which characterizes
//! `TableLike` traits which contain a single field, which is textual and the
//! primary key of the table. Furthermore, there must be a root table which
//! references this table via a foreign key.

use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike, TableLike};

/// Trait for tables that contain a single textual primary key field
/// and are referenced by a root table via a foreign key.
pub trait TableListLike: TableLike {
    /// Returns whether the table conforms to the `TableListLike` structure.
    ///
    /// # Arguments
    ///
    /// * `database` - The database context to check foreign key references.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE valid_list (name TEXT PRIMARY KEY);
    /// CREATE TABLE root (id INT PRIMARY KEY, list_name TEXT, FOREIGN KEY(list_name) REFERENCES valid_list(name));
    /// CREATE TABLE not_root (id INT PRIMARY KEY, list_name TEXT, FOREIGN KEY(list_name) REFERENCES valid_list(name));
    /// CREATE TABLE child_of_root (id INT PRIMARY KEY REFERENCES root(id));
    ///
    /// CREATE TABLE two_columns (name TEXT PRIMARY KEY, other INT);
    /// CREATE TABLE root_two (id INT PRIMARY KEY, list_name TEXT, FOREIGN KEY(list_name) REFERENCES two_columns(name));
    ///
    /// CREATE TABLE not_textual (id INT PRIMARY KEY);
    /// CREATE TABLE root_not_textual (root_id INT PRIMARY KEY, ref_id INT, FOREIGN KEY(ref_id) REFERENCES not_textual(id));
    ///
    /// CREATE TABLE orphan (name TEXT PRIMARY KEY);
    /// CREATE TABLE not_root2 (id INT PRIMARY KEY, list_name TEXT, FOREIGN KEY(list_name) REFERENCES orphan(name));
    /// "#,
    /// )?;
    ///
    /// let valid_list = db.table(None, "valid_list").unwrap();
    /// assert!(valid_list.is_table_list_like(&db));
    ///
    /// let two_columns = db.table(None, "two_columns").unwrap();
    /// assert!(!two_columns.is_table_list_like(&db));
    ///
    /// let not_textual = db.table(None, "not_textual").unwrap();
    /// assert!(!not_textual.is_table_list_like(&db));
    ///
    /// let orphan = db.table(None, "orphan").unwrap();
    /// assert!(!orphan.is_table_list_like(&db));
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn is_table_list_like(&self, database: &Self::DB) -> bool {
        // We expect exactly one column.
        if self.number_of_columns(database) != 1 {
            return false;
        }
        // We retrieve that column.
        let Some(column) = self.columns(database).next() else {
            return false;
        };
        // We check if that column is the primary key && textual.
        if !column.is_primary_key(database) || !column.is_textual(database) {
            return false;
        }

        // Check if there is at least one foreign key referencing this table
        database.root_tables().any(|table| table.refers_to(database, self.borrow()))
    }

    /// Iterates over the columns of the table which refer to table-list tables.
    ///
    /// # Arguments
    ///
    /// * `database` - The database context to check foreign key references.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE valid_list (name TEXT PRIMARY KEY);
    /// CREATE TABLE root (id INT PRIMARY KEY, list_name TEXT, FOREIGN KEY(list_name) REFERENCES valid_list(name));
    /// CREATE TABLE child_of_root (id INT PRIMARY KEY REFERENCES root(id));
    /// "#,
    /// )?;
    ///
    /// let root_table = db.table(None, "root").unwrap();
    /// let referring_columns = root_table.columns_referring_to_table_lists(&db).collect::<Vec<_>>();
    /// assert_eq!(referring_columns.len(), 1);
    /// assert_eq!(referring_columns[0].column_name(), "list_name");
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn columns_referring_to_table_lists<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column> {
        self.columns(database).filter(move |column| {
            column.foreign_keys(database).any(|fk| {
                let referenced_table = fk.referenced_table(database);
                referenced_table.is_table_list_like(database)
            })
        })
    }
}

impl<T> TableListLike for T where T: TableLike {}
