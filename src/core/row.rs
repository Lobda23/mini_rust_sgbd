//! Core row definitions for the database engine.
//!
//! This module defines the `Row` type, representing a single row
//! of data in a table. Each `Row` stores values corresponding exactly
//! to a [`Schema`] and validates types and lengths.

use crate::core::types::Value;
use crate::core::schema::Schema;
use crate::core::error::SqlError;

/// Represents a single row in a table.
///
/// A `Row` contains a vector of [`Value`]s that **exactly match** the
/// number and type of columns defined in a [`Schema`].
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    values: Vec<Value>,
}

impl Row {
    /// Constructs a new `Row` from a list of values and a schema reference.
    ///
    /// # Validation
    /// - Number of values must equal number of columns.
    /// - Each value's type must match the corresponding column's type.
    ///
    /// # Arguments
    /// - `values`: vector of [`Value`]s
    /// - `schema`: reference to the corresponding [`Schema`]
    ///
    /// # Returns
    /// `Ok(Row)` if valid, otherwise `Err(SqlError::Core)` describing the problem.
    ///
    /// # Example
    /// ```
    /// use mini_rust_sgbd::core::types::{Value, ColumnName, DataType};
    /// use mini_rust_sgbd::core::schema::{Column, Schema};
    /// use mini_rust_sgbd::core::row::Row;
    ///
    /// let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
    /// let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
    /// let schema = Schema::try_new(vec![col1, col2]).unwrap();
    ///
    /// let row = Row::from_values(vec![Value::Int(1), Value::Text("Alice".to_string())], &schema).unwrap();
    /// ```
    pub fn from_values(values: Vec<Value>, schema: &Schema) -> Result<Self, SqlError> {
        if values.len() != schema.columns().len() {
            return Err(SqlError::new_core(&format!(
                "Row has {} values but schema has {} columns",
                values.len(),
                schema.columns().len()
            )));
        }

        for (i, (value, column)) in values.iter().zip(schema.columns()).enumerate() {
            if !column.dtype.matches(value) {
                return Err(SqlError::new_core(&format!(
                    "Type mismatch at column {}: expected {:?}, got {:?}",
                    i,
                    column.dtype,
                    value
                )));
            }
        }

        Ok(Row { values })
    }

    /// Returns a reference to the values of the row.
    pub fn values(&self) -> &Vec<Value> {
        &self.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Value, ColumnName, DataType};
    use crate::core::schema::{Column, Schema};
    use crate::core::error::SqlError;

    #[test]
    fn row_valid() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
        let schema = Schema::try_new(vec![col1, col2]).unwrap();

        let values = vec![Value::Int(1), Value::Text("Alice".to_string())];
        let row = Row::from_values(values.clone(), &schema).unwrap();
        assert_eq!(row.values(), &values);
    }

    #[test]
    fn row_length_mismatch() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
        let schema = Schema::try_new(vec![col1, col2]).unwrap();

        let values = vec![Value::Int(1)];
        let result = Row::from_values(values, &schema);
        assert!(matches!(result, Err(SqlError::Core(_))));
    }

    #[test]
    fn row_type_mismatch() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
        let schema = Schema::try_new(vec![col1, col2]).unwrap();

        let values = vec![Value::Text("1".to_string()), Value::Text("Alice".to_string())];
        let result = Row::from_values(values, &schema);
        assert!(matches!(result, Err(SqlError::Core(_))));
    }
}
