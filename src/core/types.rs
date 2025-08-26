//! Core data types and table/column abstractions for the database engine.
//!
//! This module defines the fundamental abstractions for data representation
//! and schema validation in the system. It includes:
//! - [`DataType`]: schema-level type of a column.
//! - [`Value`]: runtime representation of stored values.
//! - [`TableName`] and [`ColumnName`]: type-safe wrappers for names to prevent misuse.

/// Schema-level type of a database column.
///
/// A `DataType` is declarative: it specifies the kind of values allowed
/// but does not store data itself.
///
/// # Examples
/// ```
/// use mini_rust_sgbd::core::types::DataType;
///
/// let dtype = DataType::Int;
/// assert_eq!(dtype, DataType::Int);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    /// 64-bit signed integer.
    Int,
    /// UTF-8 text string.
    Text,
}

impl DataType {
    /// Checks if a runtime [`Value`] matches this schema [`DataType`].
    ///
    /// Returns `true` if compatible, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use mini_rust_sgbd::core::types::{DataType, Value};
    ///
    /// let dtype = DataType::Int;
    /// assert!(dtype.matches(&Value::Int(42)));
    /// assert!(!dtype.matches(&Value::Text("foo".to_string())));
    /// ```
    pub fn matches(&self, value: &Value) -> bool {
        matches!((self, value),
            (DataType::Int, Value::Int(_)) |
            (DataType::Text, Value::Text(_))
        )
    }

    /// Convenience constructor for `DataType::Int`.
    pub fn new_int() -> Self { DataType::Int }

    /// Convenience constructor for `DataType::Text`.
    pub fn new_text() -> Self { DataType::Text }
}

/// Runtime value stored in the database.
///
/// Unlike [`DataType`], `Value` represents actual data.
///
/// # Examples
/// ```
/// use mini_rust_sgbd::core::types::Value;
///
/// let int_val = Value::Int(100);
/// let txt_val = Value::Text("Alice".to_string());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// 64-bit signed integer.
    Int(i64),
    /// UTF-8 text string.
    Text(String),
}

impl Value {
    /// Creates a new integer value.
    pub fn new_int(val: i64) -> Self { Value::Int(val) }

    /// Creates a new text value.
    pub fn new_text(val: String) -> Self { Value::Text(val) }
}

/// Type-safe wrapper for table names.
///
/// Table names must:
/// - Be non-empty
/// - Start with a letter
/// - Contain only ASCII letters, digits, or underscores
/// - Not contain spaces
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableName(String);

impl TableName {
    /// Creates a new `TableName` if valid.
    ///
    /// # Errors
    ///
    /// Returns an error string if the name is invalid.
    pub fn new(name: &str) -> Result<Self, String> {
        validate_name("Table", name)?;
        Ok(Self(name.to_string()))
    }

    /// Returns the table name as a string slice.
    pub fn as_str(&self) -> &str { &self.0 }
}

/// Type-safe wrapper for column names.
///
/// Column names must:
/// - Be non-empty
/// - Start with a letter
/// - Contain only ASCII letters, digits, or underscores
/// - Not contain spaces
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnName(String);

impl ColumnName {
    /// Creates a new `ColumnName` if valid.
    ///
    /// # Errors
    ///
    /// Returns an error string if the name is invalid.
    pub fn new(name: &str) -> Result<Self, String> {
        validate_name("Column", name)?;
        Ok(Self(name.to_string()))
    }

    /// Returns the column name as a string slice.
    pub fn as_str(&self) -> &str { &self.0 }
}

/// Validates a table or column name.
///
/// Rules:
/// - Non-empty
/// - Starts with ASCII letter
/// - Only ASCII letters, digits, or underscores
/// - No spaces
fn validate_name(kind: &str, name: &str) -> Result<(), String> {
    let mut chars = name.chars();
    let first = chars.next().ok_or_else(|| format!("{} name cannot be empty", kind))?;

    if !first.is_ascii_alphabetic() {
        return Err(format!("{} name must start with a letter", kind));
    }

    for c in chars {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {},
            ' ' => return Err(format!("{} name cannot contain spaces", kind)),
            _ => return Err(format!("{} name can only contain ASCII letters, digits, or underscores", kind)),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_table_name() {
        assert!(TableName::new("users").is_ok());
        assert!(TableName::new("").is_err());
        assert!(TableName::new("1users").is_err());
        assert!(TableName::new("user name").is_err());
        assert!(TableName::new("user!").is_err());
    }

    #[test]
    fn validate_column_name() {
        assert!(ColumnName::new("id").is_ok());
        assert!(ColumnName::new("").is_err());
        assert!(ColumnName::new("1abc").is_err());
        assert!(ColumnName::new("na me").is_err());
        assert!(ColumnName::new("name!").is_err());
    }

    #[test]
    fn datatype_matches_value() {
        assert!(DataType::Int.matches(&Value::Int(42)));
        assert!(!DataType::Int.matches(&Value::Text("foo".to_string())));
        assert!(DataType::Text.matches(&Value::Text("foo".to_string())));
        assert!(!DataType::Text.matches(&Value::Int(0)));
    }
}
