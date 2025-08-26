//! Error handling module for the SQL engine.
//!
//! This module defines the core error types and conventions used throughout
//! the database engine. It provides:
//! - [`SqlError`]: represents errors in the SQL engine (currently only core errors).
//! - [`SqlResult<T>`]: a type alias for `Result<T, SqlError>` to standardize return types.
//!
//! # Design Goals
//! - Uniform error reporting across modules (core, parser, executor, storage).
//! - Easy propagation of errors using `?` operator.
//! - Extensible for future error categories.

/// Represents errors that can occur in the SQL engine.
///
/// Currently contains a single variant `Core` for core subsystem errors.
/// Can be extended in the future with parser, executor, or storage errors.
///
/// # Example
/// ```
/// use mini_rust_sgbd::core::error::SqlError;
///
/// let error = SqlError::Core { message: "Critical failure".to_string() };
/// assert_eq!(error.message(), "Critical failure");
/// ```
#[derive(Debug, PartialEq)]
pub enum SqlError {
    /// Core-level error with a descriptive message.
    Core { message: String },
}

impl SqlError {
    /// Creates a new core error with a specific message.
    ///
    /// # Arguments
    /// * `message` - Description of the error.
    ///
    /// # Example
    /// ```
    /// use mini_rust_sgbd::core::error::SqlError;
    /// let err = SqlError::new_core("critical failure");
    /// assert_eq!(err.message(), "critical failure");
    /// ```
    pub fn new_core(message: &str) -> Self {
        SqlError::Core {
            message: message.to_string(),
        }
    }

    /// Returns the error message.
    ///
    /// # Returns
    /// A string slice describing the error.
    ///
    /// # Example
    /// ```
    /// use mini_rust_sgbd::core::error::SqlError;
    /// let err = SqlError::new_core("test error");
    /// assert_eq!(err.message(), "test error");
    /// ```
    pub fn message(&self) -> &str {
        match self {
            SqlError::Core { message } => message,
        }
    }
}

/// Type alias for results in the SQL engine.
///
/// Standardizes all function return types to `Result<T, SqlError>`.
///
/// # Example
/// ```
/// use mini_rust_sgbd::core::error::{SqlError, SqlResult};
///
/// fn operation() -> SqlResult<i32> {
///     Err(SqlError::new_core("operation failed"))
/// }
///
/// let result = operation();
/// assert!(result.is_err());
/// ```
pub type SqlResult<T> = Result<T, SqlError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_core_stores_message() {
        let error = SqlError::new_core("core subsystem crashed");
        assert_eq!(error.message(), "core subsystem crashed");
    }

    #[test]
    fn test_core_error_equality() {
        let e1 = SqlError::new_core("same message");
        let e2 = SqlError::new_core("same message");
        assert_eq!(e1, e2);
    }

    #[test]
    fn test_sqlresult_ok() {
        let result: SqlResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_sqlresult_err() {
        let result: SqlResult<i32> = Err(SqlError::new_core("failure"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message(), "failure");
    }

    /// Helper function to test propagation with `?` operator.
    fn failable_function() -> SqlResult<i32> {
        Err(SqlError::new_core("propagation test"))
    }

    #[test]
    fn test_error_propagation() {
        fn wrapper() -> SqlResult<i32> {
            let _x = failable_function()?; // propagate error
            Ok(1)
        }

        let result = wrapper();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message(), "propagation test");
    }
}
