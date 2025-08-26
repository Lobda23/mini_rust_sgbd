//! Core database container for the SQL engine.
//!
//! This module defines the `Database` type, which manages all tables
//! in the system. It ensures that table names are unique and provides
//! convenient access to tables, both immutable and mutable.

use std::collections::HashMap;
use crate::core::types::TableName;
use crate::core::schema::Schema;
use crate::core::table::Table;
use crate::core::error::{SqlError, SqlResult};

/// Represents a database containing multiple tables.
///
/// Ensures that table names are unique and provides methods
/// to create and access tables.
#[derive(Debug)]
pub struct Database {
    tables: HashMap<TableName, Table>,
}

impl Database {
    /// Creates a new empty database.
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    /// Creates a new table in the database.
    ///
    /// # Arguments
    /// - `name`: the table's type-safe name
    /// - `schema`: the table's schema
    ///
    /// # Returns
    /// `Ok(&Table)` if creation succeeds, otherwise `Err(SqlError)` if a table
    /// with the same name already exists.
    pub fn create_table(&mut self, name: TableName, schema: Schema) -> SqlResult<&Table> {
        if self.tables.contains_key(&name) {
            return Err(SqlError::new_core(&format!(
                "Table with name '{}' already exists",
                name.as_str()
            )));
        }

        let table = Table::create(name.clone(), schema);
        self.tables.insert(name.clone(), table);
        Ok(self.tables.get(&name).unwrap())
    }

    /// Returns an immutable reference to a table by name.
    pub fn table(&self, name: &TableName) -> Option<&Table> {
        self.tables.get(name)
    }

    /// Returns a mutable reference to a table by name.
    pub fn table_mut(&mut self, name: &TableName) -> Option<&mut Table> {
        self.tables.get_mut(name)
    }

    /// Returns the number of tables in the database.
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{TableName, ColumnName, DataType, Value};
    use crate::core::schema::{Column, Schema};
    use crate::core::row::Row;

    #[test]
    fn create_table_success() {
        let mut db = Database::new();

        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let schema = Schema::try_new(vec![col1]).unwrap();
        let table_name = TableName::new("users").unwrap();

        let table = db.create_table(table_name.clone(), schema).unwrap();
        assert_eq!(table.name().as_str(), "users");
        assert_eq!(db.table_count(), 1);
    }

    #[test]
    fn create_table_duplicate_fails() {
        let mut db = Database::new();

        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let schema = Schema::try_new(vec![col1]).unwrap();
        let table_name = TableName::new("users").unwrap();

        db.create_table(table_name.clone(), schema.clone()).unwrap();
        let result = db.create_table(table_name.clone(), schema);
        assert!(result.is_err());
    }

    #[test]
    fn access_existing_table() {
        let mut db = Database::new();

        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let schema = Schema::try_new(vec![col1]).unwrap();
        let table_name = TableName::new("users").unwrap();

        db.create_table(table_name.clone(), schema).unwrap();

        let table_ref = db.table(&table_name).unwrap();
        assert_eq!(table_ref.name().as_str(), "users");

        let table_mut_ref = db.table_mut(&table_name).unwrap();
        assert_eq!(table_mut_ref.name().as_str(), "users");
    }
}
