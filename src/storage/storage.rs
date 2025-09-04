//! Storage: persistence of tables to and from disk.
//!
//! Responsibilities:
//! - Serialize a [`Table`] into JSON and save it to a file
//! - Load a [`Table`] from a JSON file
//! - Remain decoupled from the core Database logic
//!
//! Design:
//! - Storage only uses serializable representations of Table, Column, and Row
//! - Core never depends on disk I/O
//!
//! Example:
//! ```ignore
//! save_table(&table, Path::new("users.json")).unwrap();
//! let table2 = load_table(Path::new("users.json")).unwrap();
//! ```

use std::fs::{File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use serde::{Serialize, Deserialize};

use crate::frontend::ast::{Column, Value};
use crate::executor::Row;
use crate::executor::Table;
use crate::frontend::token::SqlError;

/// Save a table to the given file path as JSON.
pub fn save_table(table: &Table, path: &Path) -> Result<(), SqlError> {
    let file = File::create(path)
        .map_err(|e| SqlError::io(format!("cannot create file: {}", e)))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &SerializableTable::from_table(table))
        .map_err(|e| SqlError::io(format!("serialization error: {}", e)))
}

/// Load a table from the given file path.
pub fn load_table(path: &Path) -> Result<Table, SqlError> {
    let file = File::open(path)
        .map_err(|e| SqlError::io(format!("cannot open file: {}", e)))?;
    let reader = BufReader::new(file);
    let st: SerializableTable = serde_json::from_reader(reader)
        .map_err(|e| SqlError::io(format!("deserialization error: {}", e)))?;
    Ok(st.into_table())
}

/// Serializable version of Table, decoupled from Core internals.
#[derive(Debug, Serialize, Deserialize)]
struct SerializableTable {
    columns: Vec<Column>,
    rows: Vec<Vec<Value>>,
}

impl SerializableTable {
    fn from_table(t: &Table) -> Self {
        Self {
            columns: t.columns.clone(),
            rows: t.rows.iter().map(|r| r.values.clone()).collect(),
        }
    }
    fn into_table(self) -> Table {
        Table {
            columns: self.columns,
            rows: self.rows.into_iter().map(|v| Row { values: v }).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::frontend::ast::{ColumnName, DataType};

    fn sample_table() -> Table {
        Table {
            columns: vec![
                Column { name: ColumnName("id".into()), ty: DataType::Int },
                Column { name: ColumnName("name".into()), ty: DataType::Text },
            ],
            rows: vec![
                Row { values: vec![Value::Int(1), Value::Str("Alice".into())] },
                Row { values: vec![Value::Int(2), Value::Str("Bob".into())] },
            ],
        }
    }

    #[test]
    fn save_and_load_roundtrip() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("users.json");

        let t1 = sample_table();
        save_table(&t1, &path).unwrap();
        let t2 = load_table(&path).unwrap();

        assert_eq!(t1.columns.len(), t2.columns.len());
        assert_eq!(t1.rows.len(), t2.rows.len());
        assert_eq!(t1.rows[0].values, t2.rows[0].values);
        assert_eq!(t1.rows[1].values, t2.rows[1].values);
    }

    #[test]
    fn error_on_missing_file() {
        let path = Path::new("nonexistent.json");
        let e = load_table(path).unwrap_err();
        assert!(matches!(e, SqlError::IoError { .. }));
    }
}
