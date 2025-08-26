//! Core schema definitions for the database engine.
//!
//! This module provides the foundational abstractions for representing
//! and managing table schemas in a type-safe and efficient manner.
//!
//! # Key Concepts
//! - **Column**: couples a [`ColumnName`] with a [`DataType`], representing
//!   a single column in a table schema.
//! - **Schema**: holds an ordered list of columns and a fast lookup map
//!   (`index_by_name`) to retrieve the index of a column by its name.
//!
//! # Usage
//! ```
//! use mini_rust_sgbd::core::types::{ColumnName, DataType};
//! use mini_rust_sgbd::core::schema::{Column, Schema};
//!
//! let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
//! let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
//! let schema = Schema::try_new(vec![col1.clone(), col2.clone()]).unwrap();
//!
//! assert_eq!(schema.index_of(&ColumnName::new("id").unwrap()), Some(0));
//! assert_eq!(schema.index_of(&ColumnName::new("name").unwrap()), Some(1));
//! ```

use std::collections::HashMap;
use crate::core::types::{ColumnName, DataType};
use crate::core::error::SqlError;

/// Represents a single column in a table schema.
///
/// Each `Column` consists of:
/// - `name`: a [`ColumnName`] for type-safe identification
/// - `dtype`: a [`DataType`] specifying allowed values
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: ColumnName,
    pub dtype: DataType,
}

impl Column {
    /// Constructs a new `Column` from a name and data type.
    ///
    /// # Arguments
    /// - `name`: type-safe column name
    /// - `dtype`: the column's data type
    ///
    /// # Returns
    /// A `Column` instance.
    pub fn new(name: ColumnName, dtype: DataType) -> Self {
        Column { name, dtype }
    }
}

/// Represents a table schema with fast column lookup.
///
/// The `Schema` maintains:
/// - `columns`: ordered list of columns for row validation
/// - `index_by_name`: map for O(1) lookup of column positions by name
#[derive(Debug, Clone)]
pub struct Schema {
    columns: Vec<Column>,
    index_by_name: HashMap<ColumnName, usize>,
}

impl Schema {
    /// Attempts to construct a `Schema` from a list of columns.
    ///
    /// # Validation
    /// Ensures no duplicate column names exist. If a duplicate is detected,
    /// returns an error describing the offending column.
    ///
    /// # Arguments
    /// - `columns`: ordered vector of [`Column`]s
    ///
    /// # Returns
    /// `Ok(Schema)` if valid, otherwise `Err(SqlError::Core)` for duplicates.
    pub fn try_new(columns: Vec<Column>) -> Result<Self, SqlError> {
        let mut index_by_name = HashMap::new();

        for (i, column) in columns.iter().enumerate() {
            if index_by_name.contains_key(&column.name) {
                return Err(SqlError::new_core(&format!(
                    "Duplicate column name: {}",
                    column.name.as_str()
                )));
            }
            index_by_name.insert(column.name.clone(), i);
        }

        Ok(Schema { columns, index_by_name })
    }

    /// Returns the index of a column by name.
    ///
    /// # Arguments
    /// - `name`: reference to a [`ColumnName`]
    ///
    /// # Returns
    /// `Some(index)` if the column exists, otherwise `None`.
    pub fn index_of(&self, name: &ColumnName) -> Option<usize> {
        self.index_by_name.get(name).copied()
    }

    /// Returns the ordered list of columns.
    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{ColumnName, DataType};
    use crate::core::error::SqlError;

    /// Test creation of a valid schema and index lookup
    #[test]
    fn schema_creation_and_index() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
        let schema = Schema::try_new(vec![col1.clone(), col2.clone()]).unwrap();

        assert_eq!(schema.index_of(&ColumnName::new("id").unwrap()), Some(0));
        assert_eq!(schema.index_of(&ColumnName::new("name").unwrap()), Some(1));
        assert_eq!(schema.columns(), &vec![col1, col2]);
    }

    /// Test schema creation fails on duplicate column names
    #[test]
    fn schema_creation_duplicate_name() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let col2 = Column::new(ColumnName::new("id").unwrap(), DataType::Text);
        let result = Schema::try_new(vec![col1, col2]);

        assert!(matches!(result, Err(SqlError::Core(_))));
    }

    /// Test index lookup for a non-existent column
    #[test]
    fn index_of_nonexistent_column() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let schema = Schema::try_new(vec![col1]).unwrap();

        assert_eq!(schema.index_of(&ColumnName::new("missing").unwrap()), None);
    }
}
