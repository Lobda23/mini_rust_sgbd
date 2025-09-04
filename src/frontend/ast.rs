//! Abstract Syntax Tree (AST) definitions for the SQL frontend.
//!
//! This module defines structs representing SQL statements and an
//! `ASTNode` enum to group all statement types. It serves as the
//! target structure for the parser.

use crate::core::types::{TableName, ColumnName, Value};
use crate::core::schema::Column;

/// Represents a CREATE TABLE statement.
///
/// # Fields
/// - `name`: the table being created
/// - `columns`: the list of columns with names and types
#[derive(Debug, Clone, PartialEq)]
pub struct CreateTableStmt {
    pub name: TableName,
    pub columns: Vec<Column>,
}

/// Represents an INSERT statement.
///
/// # Fields
/// - `table`: the table into which values are inserted
/// - `values`: the row values to insert
#[derive(Debug, Clone, PartialEq)]
pub struct InsertStmt {
    pub table: TableName,
    pub values: Vec<Value>,
}

/// Represents a SELECT statement.
///
/// # Fields
/// - `table`: the table being queried
/// - `columns`: optional list of column names to select; `None` means all columns
#[derive(Debug, Clone, PartialEq)]
pub struct SelectStmt {
    pub table: TableName,
    pub columns: Option<Vec<ColumnName>>,
}

/// Enum grouping all SQL statements into a single AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    CreateTable(CreateTableStmt),
    Insert(InsertStmt),
    Select(SelectStmt),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{TableName, ColumnName, DataType};
    use crate::core::schema::Column;
    use crate::core::types::Value;

    #[test]
    fn create_table_stmt_struct() {
        let col1 = Column::new(ColumnName::new("id").unwrap(), DataType::Int);
        let col2 = Column::new(ColumnName::new("name").unwrap(), DataType::Text);
        let stmt = CreateTableStmt {
            name: TableName::new("users").unwrap(),
            columns: vec![col1.clone(), col2.clone()],
        };
        assert_eq!(stmt.columns.len(), 2);
        assert_eq!(stmt.name.as_str(), "users");
    }

    #[test]
    fn insert_stmt_struct() {
        let stmt = InsertStmt {
            table: TableName::new("users").unwrap(),
            values: vec![Value::Int(1), Value::Text("Alice".to_string())],
        };
        assert_eq!(stmt.values.len(), 2);
        assert_eq!(stmt.table.as_str(), "users");
    }

    #[test]
    fn select_stmt_struct() {
        let stmt = SelectStmt {
            table: TableName::new("users").unwrap(),
            columns: Some(vec![ColumnName::new("id").unwrap()]),
        };
        assert_eq!(stmt.columns.as_ref().unwrap().len(), 1);
        assert_eq!(stmt.table.as_str(), "users");
    }

    #[test]
    fn ast_node_enum() {
        let create = ASTNode::CreateTable(CreateTableStmt {
            name: TableName::new("users").unwrap(),
            columns: vec![],
        });
        let insert = ASTNode::Insert(InsertStmt {
            table: TableName::new("users").unwrap(),
            values: vec![],
        });
        let select = ASTNode::Select(SelectStmt {
            table: TableName::new("users").unwrap(),
            columns: None,
        });

        match create {
            ASTNode::CreateTable(_) => {}
            _ => panic!("Expected CreateTable variant"),
        }
        match insert {
            ASTNode::Insert(_) => {}
            _ => panic!("Expected Insert variant"),
        }
        match select {
            ASTNode::Select(_) => {}
            _ => panic!("Expected Select variant"),
        }
    }
}
