

use rusqlite::{Connection, NO_PARAMS, OpenFlags, Result, types::ToSql};
use crate::benchy::benchmark::BenchmarkSubmission;

macro_rules! DB_DEFAULTS {
    (DB_PATH) => ("./benchy.db");
    (DB_TBL) => (
        "CREATE TABLE submissions (
            sub_id SERIAL PRIMARY KEY,
            ident VARCHAR(32),
            data TEXT
        );"
    );
    (DB_GET_SUBS) => (
        "SELECT * FROM submissions;";
    );
    (DB_INSERT_SUB) => (
        "INSERT INTO submissions
            VALUES (?1, ?2);"
    )
}



///
/// Establishes a connection with a local SQLIte file
/// If a file does not exist, it will create one using
/// the default database schema.
pub fn establish() -> Connection {
    match Connection::open_with_flags(DB_DEFAULTS!(DB_PATH),
        OpenFlags::SQLITE_OPEN_READ_WRITE) {
            Ok(c) => {
                c
            },
            _ => {
                create()
            }
    }
}

/// TODO: Fix unwrap() call, attempt to always guarantee that a db will be created
pub fn create() -> Connection {
    let c = Connection::open_with_flags(DB_DEFAULTS!(DB_PATH),
        OpenFlags::SQLITE_OPEN_CREATE).unwrap();
    c.execute(DB_DEFAULTS!(DB_TBL), NO_PARAMS).unwrap();
    c
}

/// Retrieves the submissions made to the database
/// If the database connection or statement failures, it will return an error
/// Routes using this function should return an empty json object with an error.
pub fn get_subs(con: &Connection) -> Result<Vec<BenchmarkSubmission>> {
    let mut stmt = con.prepare(DB_DEFAULTS!(DB_GET_SUBS))?;
    let rows = stmt.query_map(NO_PARAMS, |row| {
        BenchmarkSubmission {
            sub_id: row.get(0),
            ident: row.get(1),
            data: row.get(2)
        }
    })?;
    let mut subs = Vec::new();
    for res in rows {
        subs.push(res?);
    }
    Ok(subs)
}


/// Inserts a submission, typically after it has been executed by the Test Runner object
/// If the execution fails it will return an error code as part of the result.
pub fn insert_sub(conn: &Connection, ident: &String, data: &String) -> Result<usize> {
    conn.execute(
        DB_DEFAULTS!(DB_INSERT_SUB),
        &[ident as &ToSql, data as &ToSql]
    )
}

