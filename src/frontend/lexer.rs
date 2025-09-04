//! Lexer module for the SQL frontend.
//!
//! This module provides a simple lexer that transforms an input SQL string
//! into a sequence of [`Token`]s. It recognizes keywords, identifiers,
//! numbers, string literals, and symbols. Spaces and simple comments
//! (starting with --) are ignored.

use crate::core::error::{SqlError, SqlResult};
use crate::frontend::token::Token;

/// List of SQL keywords recognized by the lexer.
const KEYWORDS: &[&str] = &[
    "SELECT", "INSERT", "UPDATE", "DELETE", "CREATE", "TABLE", "VALUES",
];

/// Symbols recognized in SQL.
const SYMBOLS: &[char] = &['(', ')', ',', ';'];

/// Lexical analysis: transform input SQL string into a vector of tokens.
///
/// # Arguments
/// * `input` - SQL query string.
///
/// # Returns
/// * `Ok(Vec<Token>)` on success
/// * `Err(SqlError)` if an invalid token is found
pub fn lexer(input: &str) -> SqlResult<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut pos = 0;

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
                pos += 1;
            }

            // Symbols
            c if SYMBOLS.contains(&c) => {
                tokens.push(Token::Symbol { value: c, pos: Some(pos) });
                chars.next();
                pos += 1;
            }

            // Number literal
            '0'..='9' => {
                let start = pos;
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                        chars.next();
                        pos += 1;
                    } else {
                        break;
                    }
                }
                let value = num_str.parse::<i64>().map_err(|_| {
                    SqlError::new_core(&format!("Invalid number at position {}", start))
                })?;
                tokens.push(Token::Number { value, pos: Some(start) });
            }

            // String literal
            '\'' => {
                let start = pos;
                chars.next(); // skip opening quote
                pos += 1;
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '\'' {
                        chars.next();
                        pos += 1;
                        break;
                    } else {
                        s.push(c);
                        chars.next();
                        pos += 1;
                    }
                }
                tokens.push(Token::String { value: s, pos: Some(start) });
            }

            // Identifier or keyword
            c if c.is_ascii_alphabetic() => {
                let start = pos;
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        word.push(c);
                        chars.next();
                        pos += 1;
                    } else {
                        break;
                    }
                }
                let word_upper = word.to_uppercase();
                if KEYWORDS.contains(&word_upper.as_str()) {
                    tokens.push(Token::Keyword { value: word_upper, pos: Some(start) });
                } else {
                    tokens.push(Token::Identifier { value: word, pos: Some(start) });
                }
            }

            _ => {
                return Err(SqlError::new_core(&format!("Unexpected character '{}' at position {}", ch, pos)));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::token::Token;

    #[test]
    fn lexer_basic_create_table() {
        let sql = "CREATE TABLE users (id Int);";
        let tokens = lexer(sql).unwrap();

        let expected = vec![
            Token::Keyword { value: "CREATE".to_string(), pos: Some(0) },
            Token::Keyword { value: "TABLE".to_string(), pos: Some(7) },
            Token::Identifier { value: "users".to_string(), pos: Some(13) },
            Token::Symbol { value: '(', pos: Some(19) },
            Token::Identifier { value: "id".to_string(), pos: Some(20) },
            Token::Identifier { value: "Int".to_string(), pos: Some(23) },
            Token::Symbol { value: ')', pos: Some(26) },
            Token::Symbol { value: ';', pos: Some(27) },
        ];

        assert_eq!(tokens, expected);
    }
}
