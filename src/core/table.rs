//! Core table definitions for the database engine.
//!
//! This module defines the `Table` type, representing a table in the database.
//! A `Table` consists of:
//! - a [`TableName`] for identification
//! - a [`Schema`] defining its columns
//! - a list of [`Row`]s storing the actual data
//!
//! The `Table` enforces that all inserted rows match the schema exactly.

use crate::core::types::TableName;
use crate::core::schema::Schema;
use crate::core::row::Row;
use crate::core::error::{SqlError, SqlResult};

/// Represents a database table.
///
/// Each `Table` contains:
/// - `name`: a [`TableName`] identifying the table
/// - `schema`: the table's [`Schema`]
/// - `rows`: the list of [`Row`]s stored
#[derive(Debug, Clone)]
pub struct Table {
    name: TableName,
    schema: Schema,
    rows: Vec<Row>,
}

impl Table {
    /// Creates a new empty table with the given name and schema.
    ///
    /// # Arguments
    /// - `name`: type-safe table name
    /// - `schema`: the schema defining the columns
    ///
    /// # Returns
    /// A new empty `Table`.
    pub fn create(name: TableName, schema: Schema) -> Self {
        Table {
            name,
            schema,
            rows: Vec::new(),
        }
    }

    /// Inserts a row into the table after validating against the schema.
    ///
    /// # Validation
    /// - Number of values must match number of columns
    /// - Each value type must match the corresponding column type
    ///
    /// # Arguments
    /// - `row`: the row to insert
    ///
    /// # Returns
    /// `Ok(())` if insertion succeeds, otherwise `Err(SqlError)` describing the problem.
    pub fn insert_checked(&mut self, row: Row) -> SqlResult<()> {
        // Validate row against schema
        if row.values().len() != self.schema.columns().len() {
            return Err(SqlError::new_core(&format!(
                "Row has {} values but schema has {} columns",
                row.values().len(),
                self.schema.columns().len()
            )));
        }

        for (i, (value, column)) in row.values().iter().zip(self.schema.columns()).enumerate() {
            if !column.dtype.matches(value) {
                return Err(SqlError::new_core(&format!(
                    "Type mismatch at column {}: expected {:?}, got {:?}",
                    i,
                    column.dtype,
                    value
                )));
            }
        }

        self.rows.push(row);
        Ok(())
    }

    /// Returns a reference to the table's rows.
    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    /// Returns the table name.
    pub fn name(&self) -> &TableName {
        &self.name
    }

    /// Returns the table schema.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{TableName, ColumnName, DataType, Value};
    use crate::core::schema::{Column, Schema};

    #[test]
    fn insert_valid_row_increases_rows() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
        let schema = Schema::try_new(vec![col1, col2]).unwrap();
        let table_name = TableName::new("users").unwrap();
        let mut table = Table::create(table_name, schema);

        let row = Row::from_values(vec![Value::Int(1), Value::Text("Alice".to_string())], table.schema()).unwrap();
        table.insert_checked(row).unwrap();

        assert_eq!(table.rows().len(), 1);
    }

    #[test]
    fn insert_row_length_mismatch_fails() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
        let schema = Schema::try_new(vec![col1, col2]).unwrap();
        let table_name = TableName::new("users").unwrap();
        let mut table = Table::create(table_name, schema);

        let row = Row::from_values(vec![Value::Int(1)], table.schema()).unwrap();
        let result = table.insert_checked(row);
        assert!(result.is_err());
    }

    #[test]
    fn insert_row_type_mismatch_fails() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
        let schema = Schema::try_new(vec![col1, col2]).unwrap();
        let table_name = TableName::new("users").unwrap();
        let mut table = Table::create(table_name, schema);

        let row = Row::from_values(vec![Value::Text("1".to_string()), Value::Text("Alice".to_string())], table.schema()).unwrap();
        let result = table.insert_checked(row);
        assert!(result.is_err());
    }
}
