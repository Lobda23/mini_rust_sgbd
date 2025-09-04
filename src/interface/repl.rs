//! REPL (Read–Eval–Print Loop) for the SQL engine.
//!
//! This module defines the interactive command-line interface
//! for running SQL queries. It connects the user input pipeline
//! (lexer → parser → executor) with a database instance and
//! prints query results back to the user.
//!
//! The REPL continues until the user types `exit`, `quit`, or
//! an EOF signal is received.

use std::io::{self, Write};
use crate::frontend::lexer::lexer;
use crate::frontend::parser::Parser;
use crate::executor::{Executor, Output, Database};

/// Runs the SQL REPL loop.
///
/// Prints a prompt (`sql>`), reads user input, processes it
/// into an AST, executes it against the provided [`Database`],
/// and prints results or errors.
///
/// # Arguments
/// - `db`: a mutable reference to the active database
///
/// # Behavior
/// - Empty lines are ignored
/// - `exit` or `quit` terminates the loop
/// - Errors are printed but do not stop the REPL
pub fn run_repl(db: &mut Database) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        // Prompt
        print!("sql> ");
        stdout.flush().unwrap();

        // Read user input
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 {
            break; // EOF
        }
        let line = line.trim();

        // Exit handling
        if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
            break;
        }
        if line.is_empty() {
            continue;
        }

        // Process pipeline: lexer → parser → executor
        match lexer(line) {
            Ok(tokens) => match Parser::parse(&tokens) {
                Ok(ast) => match Executor::execute(ast, db) {
                    Ok(out) => print_output(out),
                    Err(e) => eprintln!("Execution error: {e}"),
                },
                Err(e) => eprintln!("Parse error: {e}"),
            },
            Err(e) => eprintln!("Lex error: {e}"),
        }
    }
}

/// Prints query results to stdout.
///
/// # Arguments
/// - `out`: the executor result, either no output or a set of rows
///
/// # Behavior
/// - For `Output::None`, prints `"OK"`
/// - For `Output::Rows`, prints each row with values separated by `|`
fn print_output(out: Output) {
    match out {
        Output::None => println!("OK"),
        Output::Rows(rows) => {
            for row in rows {
                let values: Vec<String> =
                    row.values.into_iter().map(|v| format!("{:?}", v)).collect();
                println!("{}", values.join(" | "));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::{ASTNode, CreateTableStmt, TableName, Column, ColumnName, DataType};

    #[test]
    fn create_table_pipeline_executes_ok() {
        let mut db = Database::new();

        // Simulate "CREATE TABLE t (id INT)"
        let stmt = CreateTableStmt {
            name: TableName("t".into()),
            columns: vec![Column {
                name: ColumnName("id".into()),
                ty: DataType::Int,
            }],
        };
        let ast = ASTNode::CreateTable(stmt);
        let out = Executor::execute(ast, &mut db).unwrap();

        assert_eq!(out, Output::None);
        assert!(db.get_table("t").is_some());
    }

    #[test]
    fn exit_and_quit_commands_match() {
        assert!("exit".eq_ignore_ascii_case("EXIT"));
        assert!("quit".eq_ignore_ascii_case("Quit"));
    }
}
