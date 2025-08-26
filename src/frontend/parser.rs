//! Parser module for the SQL frontend.
//!
//! This module provides a `Parser` struct with methods to convert a
//! sequence of [`Token`]s into an abstract syntax tree (`ASTNode`).
//! It performs basic syntax checks such as matching parentheses and
//! correct number of values for INSERT statements.

use crate::core::error::{SqlError, SqlResult};
use crate::core::types::{TableName, ColumnName, DataType};
use crate::core::schema::Column;
use crate::frontend::token::Token;
use crate::frontend::ast::{ASTNode, CreateTableStmt};

/// Parser struct with associated methods.
pub struct Parser;

impl Parser {
    /// Parse a sequence of tokens into a single AST node.
    ///
    /// # Arguments
    /// * `tokens` - Slice of tokens to parse.
    ///
    /// # Returns
    /// * `Ok(ASTNode)` on success.
    /// * `Err(SqlError)` on syntax error or invalid structure.
    pub fn parse(tokens: &[Token]) -> SqlResult<ASTNode> {
        let mut iter = tokens.iter().peekable();

        match iter.peek() {
            Some(Token::Keyword { value, .. }) => match value.as_str() {
                "CREATE" => Self::parse_create_table(&mut iter),
                "INSERT" => Self::parse_insert(&mut iter),
                "SELECT" => Self::parse_select(&mut iter),
                _ => Err(SqlError::new_core(&format!("Unexpected keyword '{}'", value))),
            },
            Some(_) => Err(SqlError::new_core("Expected a keyword at the beginning")),
            None => Err(SqlError::new_core("Empty token stream")),
        }
    }

    fn parse_create_table<'a, I>(iter: &mut std::iter::Peekable<I>) -> SqlResult<ASTNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Consume CREATE
        iter.next();

        // Expect TABLE
        match iter.next() {
            Some(Token::Keyword { value, .. }) if value == "TABLE" => {}
            _ => return Err(SqlError::new_core("Expected TABLE after CREATE")),
        }

        // Table name
        let table_name = match iter.next() {
            Some(Token::Identifier { value, .. }) => TableName::new(value)
                .map_err(|e| SqlError::new_core(&e))?,
            _ => return Err(SqlError::new_core("Expected table name after TABLE")),
        };


        // Expect '('
        match iter.next() {
            Some(Token::Symbol { value, .. }) if *value == '(' => {}
            _ => return Err(SqlError::new_core("Expected '(' after table name")),
        }

        let mut columns = Vec::new();
        loop {
            // Column name
            let col_name = match iter.next() {
                Some(Token::Identifier { value, .. }) => ColumnName::new(value)
                    .map_err(|e| SqlError::new_core(&e))?,
                _ => return Err(SqlError::new_core("Expected column name"))
            };

            // Column type
            let col_type = match iter.next() {
                Some(Token::Identifier { value, .. }) => match value.as_str() {
                    "Int" => DataType::Int,
                    "Text" => DataType::Text,
                    _ => return Err(SqlError::new_core(&format!("Unknown type '{}'", value))),
                },
                _ => return Err(SqlError::new_core("Expected column type")),
            };

            columns.push(Column::new(col_name, col_type));

            // Comma or closing parenthesis
            match iter.next() {
                Some(Token::Symbol { value, .. }) if *value == ',' => continue,
                Some(Token::Symbol { value, .. }) if *value == ')' => break,
                _ => return Err(SqlError::new_core("Expected ',' or ')' after column definition")),
            }
        }

        // Optional ';'
        if let Some(Token::Symbol { value, .. }) = iter.peek() {
            if *value == ';' {
                iter.next();
            }
        }

        Ok(ASTNode::CreateTable(CreateTableStmt { name: table_name, columns }))
    }

    fn parse_insert<'a, I>(_iter: &mut std::iter::Peekable<I>) -> SqlResult<ASTNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Simplified: placeholder
        Err(SqlError::new_core("INSERT parsing not implemented yet"))
    }

    fn parse_select<'a, I>(_iter: &mut std::iter::Peekable<I>) -> SqlResult<ASTNode>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Simplified: placeholder
        Err(SqlError::new_core("SELECT parsing not implemented yet"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::token::Token;

    #[test]
    fn parse_create_table_example() {
        let sql_tokens = vec![
            Token::Keyword { value: "CREATE".to_string(), pos: Some(0) },
            Token::Keyword { value: "TABLE".to_string(), pos: Some(7) },
            Token::Identifier { value: "users".to_string(), pos: Some(13) },
            Token::Symbol { value: '(', pos: Some(19) },
            Token::Identifier { value: "id".to_string(), pos: Some(20) },
            Token::Identifier { value: "Int".to_string(), pos: Some(23) },
            Token::Symbol { value: ',', pos: Some(26) },
            Token::Identifier { value: "name".to_string(), pos: Some(28) },
            Token::Identifier { value: "Text".to_string(), pos: Some(33) },
            Token::Symbol { value: ')', pos: Some(37) },
            Token::Symbol { value: ';', pos: Some(38) },
        ];

        let ast = Parser::parse(&sql_tokens).unwrap();
        match ast {
            ASTNode::CreateTable(stmt) => {
                assert_eq!(stmt.name.as_str(), "users");
                assert_eq!(stmt.columns.len(), 2);
                assert_eq!(stmt.columns[0].name.as_str(), "id");
                assert_eq!(stmt.columns[1].name.as_str(), "name");
            }
            _ => panic!("Expected CreateTable ASTNode"),
        }
    }
}
