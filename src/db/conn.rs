

use rusqlite::{Connection, NO_PARAMS, OpenFlags, Result, types::ToSql};
use crate::benchy::benchmark::BenchmarkSubmission;

macro_rules! DB_DEFAULTS {
    (DB_PATH) => ("./benchy.db");
    (DB_TBL) => (
        "CREATE TABLE submissions (
            sub_id INTEGER PRIMARY KEY AUTOINCREMENT,
            ident TEXT,
            data TEXT
        );"
    );
    (DB_GET_SUBS) => ("SELECT DISTINCT ident, MAX(sub_id), data FROM submissions GROUP BY ident;");
    (DB_INSERT_SUB) => ("INSERT INTO submissions(ident, data) VALUES (?1, ?2);")
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
/// Should be able to create an in memory database as worst case scenario
pub fn create() -> Connection {
    if let Ok(c) = Connection::open(DB_DEFAULTS!(DB_PATH)) {
        match c.execute(DB_DEFAULTS!(DB_TBL), NO_PARAMS) {
            Ok(_) => { println!("Table was created"); },
            Err(_) => {println!("Table was not created"); }
        }
        return c;
    } else {
        //At this point, *Force* panic if database cannot open
        return Connection::open_in_memory().unwrap();
    }
}

/// Retrieves the submissions made to the database
/// If the database connection or statement failures, it will return an error
/// Routes using this function should return an empty json object with an error.
pub fn get_subs(con: &Connection) -> Result<Vec<BenchmarkSubmission>> {
    let mut stmt = con.prepare(DB_DEFAULTS!(DB_GET_SUBS))?;
    let rows = stmt.query_map(NO_PARAMS, |row| {
        BenchmarkSubmission {
            sub_id: row.get(1),
            ident: row.get(0),
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

