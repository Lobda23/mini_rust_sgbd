//! Core data types for the database engine.
//!
//! This module defines the fundamental abstractions for data
//! representation in the system. It distinguishes between:
//! - [`DataType`]: the *schema-level type* of a column.
//! - [`Value`]: the *runtime representation* of stored values.
//!
//! These types are the foundation for ensuring consistency between
//! declared schemas and inserted rows.

/// Represents the supported types of a database column.
///
/// A `DataType` is **declarative**: it specifies the kind of values
/// that are allowed in a column, but does not store values itself.
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
    /// 64-bit signed integer values.
    Int,

    /// Variable-length UTF-8 text values.
    Text,
}

impl DataType {
    /// Checks whether this [`DataType`] matches a given [`Value`].
    ///
    /// # Arguments
    /// * `value` - A reference to the `Value` being validated.
    ///
    /// # Returns
    /// `true` if the value is compatible with this type, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use mini_rust_sgbd::core::types::{DataType, Value};
    ///
    /// let dtype = DataType::Int;
    /// assert!(dtype.matches(&Value::Int(42)));
    /// assert!(!dtype.matches(&Value::Text("wrong".to_string())));
    /// ```
    pub fn matches(&self, value: &Value) -> bool {
        match (self, value) {
            (DataType::Int, Value::Int(_)) => true,
            (DataType::Text, Value::Text(_)) => true,
            _ => false,
        }
    }

    /// Convenience constructor for `DataType::Int`.
    pub fn new_int() -> Self {
        DataType::Int
    }

    /// Convenience constructor for `DataType::Text`.
    pub fn new_text() -> Self {
        DataType::Text
    }
}

/// Represents an actual value stored in the database.
///
/// Unlike [`DataType`], which is schema metadata,
/// `Value` represents **concrete runtime data**.
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
    /// A 64-bit signed integer.
    Int(i64),

    /// A UTF-8 text string.
    Text(String),
}

impl Value {
    /// Construct a new integer value.
    ///
    /// # Arguments
    /// * `val` - An `i64` integer.
    ///
    /// # Returns
    /// A [`Value::Int`] containing the provided number.
    ///
    /// # Example
    /// ```
    /// use mini_rust_sgbd::core::types::Value;
    ///
    /// let v = Value::new_int(123);
    /// assert_eq!(v, Value::Int(123));
    /// ```
    pub fn new_int(val: i64) -> Self {
        Value::Int(val)
    }

    /// Construct a new text value.
    ///
    /// # Arguments
    /// * `val` - A `String` containing the text.
    ///
    /// # Returns
    /// A [`Value::Text`] containing the provided string.
    ///
    /// # Example
    /// ```
    /// use mini_rust_sgbd::core::types::Value;
    ///
    /// let v = Value::new_text("hello".to_string());
    /// assert_eq!(v, Value::Text("hello".to_string()));
    /// ```
    pub fn new_text(val: String) -> Self {
        Value::Text(val)
    }
}