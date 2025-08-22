//! Error handling module for the SQL engine.
//!
//! This module provides the [`SqlError`] type and the [`SqlResult`] alias,
//! which are used throughout the database engine to ensure consistent
//! error reporting and propagation.
//!
//! # Overview
//!
//! - [`SqlError`]: Represents different categories of errors (currently only
//!   core-related errors, but extensible to parser, executor, or storage).
//! - [`SqlResult`]: A `Result<T, SqlError>` alias to standardize return types.
//!
//! These abstractions allow the engine to evolve while maintaining a
//! uniform error-handling strategy.

/// Represents errors that can occur in the SQL engine.
///
/// This enum currently contains a single variant (`Core`) for errors
/// originating in the core subsystem. Future extensions can add
/// parser, executor, or storage-specific error variants.
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
    /// Represents a core-level error with a descriptive message.
    Core { message: String },
}

impl SqlError {
    /// Create a new `SqlError` of type `Core` with a specific message.
    ///
    /// # Arguments
    ///
    /// * `message` - A string slice describing the error.
    ///
    /// # Example
    ///
    /// ```
    /// use mini_rust_sgbd::core::error::SqlError;
    ///
    /// let err = SqlError::new_core("critical failure in the core module");
    /// assert_eq!(err.message(), "critical failure in the core module");
    /// ```
    pub fn new_core(message: &str) -> Self {
        SqlError::Core {
            message: message.to_string(),
        }
    }

    /// Return the error message contained in the `SqlError`.
    ///
    /// # Returns
    ///
    /// A string slice (`&str`) representing the error message.
    ///
    /// # Example
    ///
    /// ```
    /// use mini_rust_sgbd::core::error::SqlError;
    ///
    /// let err = SqlError::new_core("test error");
    /// assert_eq!(err.message(), "test error");
    /// ```
    pub fn message(&self) -> &str {
        match self {
            SqlError::Core { message } => message,
        }
    }
}

/// Type alias for results returned by SQL-related functions.
///
/// Ensures a consistent error type across the database engine.
///
/// # Example
///
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

/// ================== Unit Tests ==================
#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that `SqlError::new_core` correctly stores the provided message.
    #[test]
    fn test_new_core_stores_message() {
        let error = SqlError::new_core("core subsystem crashed");
        assert_eq!(error.message(), "core subsystem crashed");
    }

    /// Ensure that two `SqlError::Core` instances with the same message are equal.
    #[test]
    fn test_core_error_equality() {
        let e1 = SqlError::new_core("same message");
        let e2 = SqlError::new_core("same message");
        assert_eq!(e1, e2);
    }

    /// Ensure that `SqlResult<T>` works correctly with `Ok`.
    #[test]
    fn test_sqlresult_ok() {
        let result: SqlResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    /// Ensure that `SqlResult<T>` works correctly with `Err`.
    #[test]
    fn test_sqlresult_err() {
        let result: SqlResult<i32> = Err(SqlError::new_core("failure"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message(), "failure");
    }

    /// Ensure that `?` operator correctly propagates `SqlError`.
    fn failable_function() -> SqlResult<i32> {
        Err(SqlError::new_core("propagation test"))
    }

    #[test]
    fn test_error_propagation() {
        fn wrapper() -> SqlResult<i32> {
            let _x = failable_function()?; // should propagate the error
            Ok(1)
        }

        let result = wrapper();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message(), "propagation test");
    }
}
