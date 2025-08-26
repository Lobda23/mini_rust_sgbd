//! Token definitions for the SQL frontend lexer.
//!
//! This module defines the `Token` enum representing all lexical tokens
//! in SQL, including keywords, identifiers, literals, and symbols.
//! Each token stores the raw value and optional position information
//! for error reporting.

/// Represents a lexical token in SQL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// SQL keywords like SELECT, INSERT, etc.
    Keyword { value: String, pos: Option<usize> },

    /// Identifiers: table names, column names, etc.
    Identifier { value: String, pos: Option<usize> },

    /// Numeric literal (integer)
    Number { value: i64, pos: Option<usize> },

    /// String literal (UTF-8)
    String { value: String, pos: Option<usize> },

    /// Symbols like (, ), ,, ;
    Symbol { value: char, pos: Option<usize> },
}

impl Token {
    /// Returns the raw value as string (for keywords, identifiers, strings, numbers)
    pub fn value(&self) -> String {
        match self {
            Token::Keyword { value, .. } => value.clone(),
            Token::Identifier { value, .. } => value.clone(),
            Token::Number { value, .. } => value.to_string(),
            Token::String { value, .. } => value.clone(),
            Token::Symbol { value, .. } => value.to_string(),
        }
    }

    /// Returns the optional position
    pub fn pos(&self) -> Option<usize> {
        match self {
            Token::Keyword { pos, .. } => *pos,
            Token::Identifier { pos, .. } => *pos,
            Token::Number { pos, .. } => *pos,
            Token::String { pos, .. } => *pos,
            Token::Symbol { pos, .. } => *pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_token_value() {
        let t = Token::Keyword { value: "SELECT".to_string(), pos: Some(0) };
        assert_eq!(t.value(), "SELECT");
        assert_eq!(t.pos(), Some(0));
    }

    #[test]
    fn identifier_token_value() {
        let t = Token::Identifier { value: "user_id".to_string(), pos: None };
        assert_eq!(t.value(), "user_id");
        assert_eq!(t.pos(), None);
    }

    #[test]
    fn number_token_value() {
        let t = Token::Number { value: 42, pos: Some(5) };
        assert_eq!(t.value(), "42");
        assert_eq!(t.pos(), Some(5));
    }

    #[test]
    fn string_token_value() {
        let t = Token::String { value: "Alice".to_string(), pos: None };
        assert_eq!(t.value(), "Alice");
        assert_eq!(t.pos(), None);
    }

    #[test]
    fn symbol_token_value() {
        let t = Token::Symbol { value: '(', pos: Some(3) };
        assert_eq!(t.value(), "(");
        assert_eq!(t.pos(), Some(3));
    }
}
