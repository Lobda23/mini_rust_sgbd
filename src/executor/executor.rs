//! Executor: applies parsed AST nodes onto the database core.
//!
//! Responsibilities:
//! - Translate AST commands into calls on the [`Database`] and [`Table`] APIs
//! - Enforce basic semantic checks before delegating to the core
//! - Return an [`Output`] for SELECT queries
//!
//! Expected Core API (to be provided elsewhere):
//! ```ignore
//! pub struct Database { /* holds tables */ }
//! pub struct Table { /* holds schema + rows */ }
//! pub struct Row { pub values: Vec<Value> }
//!
//! impl Database {
//!     pub fn create_table(&mut self, name: &str, cols: Vec<Column>) -> Result<(), SqlError>;
//!     pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table>;
//!     pub fn get_table(&self, name: &str) -> Option<&Table>;
//! }
//!
//! impl Table {
//!     pub fn insert_checked(&mut self, values: Vec<Value>) -> Result<(), SqlError>;
//!     pub fn rows(&self) -> &[Row];
//! }
//! ```

use crate::frontend::ast::*;
use crate::frontend::token::SqlError;

/// Output returned by the executor.
///
/// - For `CREATE TABLE` and `INSERT`: typically just confirmation.
/// - For `SELECT`: rows of values.
#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    None,           // e.g. CREATE or INSERT
    Rows(Vec<Row>), // SELECT results
}

/// Simplified row type (in Core this would be more detailed).
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub values: Vec<Value>,
}

/// Executor translates AST into Core calls.
pub struct Executor;

impl Executor {
    /// Main entry point: execute one AST node on the database.
    pub fn execute(ast: ASTNode, db: &mut Database) -> Result<Output, SqlError> {
        match ast {
            ASTNode::CreateTable(stmt) => Self::exec_create(stmt, db),
            ASTNode::Insert(stmt) => Self::exec_insert(stmt, db),
            ASTNode::Select(stmt) => Self::exec_select(stmt, db),
        }
    }

    fn exec_create(stmt: CreateTableStmt, db: &mut Database) -> Result<Output, SqlError> {
        db.create_table(&stmt.name.0, stmt.columns)?;
        Ok(Output::None)
    }

    fn exec_insert(stmt: InsertStmt, db: &mut Database) -> Result<Output, SqlError> {
        let table = db
            .get_table_mut(&stmt.table.0)
            .ok_or_else(|| SqlError::parse(format!("unknown table: {}", stmt.table.0), dummy_span()))?;
        table.insert_checked(stmt.values)?;
        Ok(Output::None)
    }

    fn exec_select(stmt: SelectStmt, db: &mut Database) -> Result<Output, SqlError> {
        let table = db
            .get_table(&stmt.table.0)
            .ok_or_else(|| SqlError::parse(format!("unknown table: {}", stmt.table.0), dummy_span()))?;
        let mut rows_out = Vec::new();
        for row in table.rows() {
            // projection: either all columns or a subset
            let projected = if let Some(cols) = &stmt.columns {
                let mut vals = Vec::new();
                for col in cols {
                    // here we assume column index lookup by name
                    let idx = table
                        .column_index(&col.0)
                        .ok_or_else(|| SqlError::parse(format!("unknown column: {}", col.0), dummy_span()))?;
                    vals.push(row.values[idx].clone());
                }
                Row { values: vals }
            } else {
                row.clone()
            };
            rows_out.push(projected);
        }
        Ok(Output::Rows(rows_out))
    }
}

/// Dummy span used for semantic errors (outside parsing).
fn dummy_span() -> crate::frontend::token::Span {
    crate::frontend::token::Span::new(0, 0, 0)
}

/// --- Stub Core API for testing only ---
/// In real code these should be in `src/core/`.
pub struct Database {
    tables: std::collections::HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Self { tables: std::collections::HashMap::new() }
    }
    pub fn create_table(&mut self, name: &str, cols: Vec<Column>) -> Result<(), SqlError> {
        if self.tables.contains_key(name) {
            return Err(SqlError::parse(format!("table already exists: {}", name), dummy_span()));
        }
        self.tables.insert(name.to_string(), Table { columns: cols, rows: Vec::new() });
        Ok(())
    }
    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.get_mut(name)
    }
    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }
}

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl Table {
    pub fn insert_checked(&mut self, values: Vec<Value>) -> Result<(), SqlError> {
        if values.len() != self.columns.len() {
            return Err(SqlError::parse(
                format!("insert arity mismatch: expected {}, got {}", self.columns.len(), values.len()),
                dummy_span(),
            ));
        }
        self.rows.push(Row { values });
        Ok(())
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.name.0 == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::*;

    fn setup_db() -> Database {
        Database::new()
    }

    #[test]
    fn create_table_adds_table() {
        let mut db = setup_db();
        let stmt = CreateTableStmt {
            name: TableName("users".into()),
            columns: vec![
                Column { name: ColumnName("id".into()), ty: DataType::Int },
                Column { name: ColumnName("name".into()), ty: DataType::Text },
            ],
        };
        let out = Executor::execute(ASTNode::CreateTable(stmt), &mut db).unwrap();
        assert_eq!(out, Output::None);
        assert!(db.get_table("users").is_some());
    }

    #[test]
    fn insert_and_select_roundtrip() {
        let mut db = setup_db();
        // create table
        Executor::execute(
            ASTNode::CreateTable(CreateTableStmt {
                name: TableName("users".into()),
                columns: vec![
                    Column { name: ColumnName("id".into()), ty: DataType::Int },
                    Column { name: ColumnName("name".into()), ty: DataType::Text },
                ],
            }),
            &mut db,
        ).unwrap();

        // insert
        Executor::execute(
            ASTNode::Insert(InsertStmt {
                table: TableName("users".into()),
                columns: None,
                values: vec![Value::Int(1), Value::Str("Alice".into())],
            }),
            &mut db,
        ).unwrap();

        // select *
        let out = Executor::execute(
            ASTNode::Select(SelectStmt {
                table: TableName("users".into()),
                columns: None,
            }),
            &mut db,
        ).unwrap();

        match out {
            Output::Rows(rows) => {
                assert_eq!(rows.len(), 1);
                assert_eq!(rows[0].values, vec![Value::Int(1), Value::Str("Alice".into())]);
            }
            _ => panic!("expected rows"),
        }
    }

    #[test]
    fn select_projection_by_column_names() {
        let mut db = setup_db();
        Executor::execute(
            ASTNode::CreateTable(CreateTableStmt {
                name: TableName("t".into()),
                columns: vec![
                    Column { name: ColumnName("a".into()), ty: DataType::Int },
                    Column { name: ColumnName("b".into()), ty: DataType::Int },
                ],
            }),
            &mut db,
        ).unwrap();

        Executor::execute(
            ASTNode::Insert(InsertStmt {
                table: TableName("t".into()),
                columns: None,
                values: vec![Value::Int(10), Value::Int(20)],
            }),
            &mut db,
        ).unwrap();

        let out = Executor::execute(
            ASTNode::Select(SelectStmt {
                table: TableName("t".into()),
                columns: Some(vec![ColumnName("b".into())]),
            }),
            &mut db,
        ).unwrap();

        match out {
            Output::Rows(rows) => {
                assert_eq!(rows[0].values, vec![Value::Int(20)]);
            }
            _ => panic!("expected rows"),
        }
    }

    #[test]
    fn errors_on_unknown_table_or_column() {
        let mut db = setup_db();
        let e = Executor::execute(
            ASTNode::Insert(InsertStmt {
                table: TableName("nosuch".into()),
                columns: None,
                values: vec![],
            }),
            &mut db,
        ).unwrap_err();
        assert!(matches!(e, SqlError::ParseError { .. }));

        // create table with 1 column
        Executor::execute(
            ASTNode::CreateTable(CreateTableStmt {
                name: TableName("t".into()),
                columns: vec![Column { name: ColumnName("a".into()), ty: DataType::Int }],
            }),
            &mut db,
        ).unwrap();

        // select non-existing column
        let e = Executor::execute(
            ASTNode::Select(SelectStmt {
                table: TableName("t".into()),
                columns: Some(vec![ColumnName("b".into())]),
            }),
            &mut db,
        ).unwrap_err();
        assert!(matches!(e, SqlError::ParseError { .. }));
    }
}
