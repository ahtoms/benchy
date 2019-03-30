
use serde::{Serialize};
use rusqlite::{Connection, NO_PARAMS, OpenFlags, Result};


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
        "SELECT * FROM submissions";
    );
}

#[derive(Serialize)]
pub struct BenchmarkSubmission {
    pub sub_id: i32,
    pub ident: String,
    pub data: String
}


///Must be file based because an in-memory connection is unable
///to be shared safely between mutliple threads (it will recreate the connection)
//TODO: address unwrap calls
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

/// TODO: Fix unwrap() call
pub fn create() -> Connection {
    let c = Connection::open_with_flags(DB_DEFAULTS!(DB_PATH),
        OpenFlags::SQLITE_OPEN_CREATE).unwrap();
    c.execute(DB_DEFAULTS!(DB_TBL), NO_PARAMS).unwrap();
    c
}

/// TODO: Fix unwrap() call on query_map
pub fn get_subs(con: &Connection) -> Vec<BenchmarkSubmission> {
    let mut stmt = con.prepare(DB_DEFAULTS!(DB_GET_SUBS)).unwrap();
    let res = stmt.query_map(NO_PARAMS, |row| {
        BenchmarkSubmission {
            sub_id: row.get(0),
            ident: row.get(1),
            data: row.get(2)
        }
    }).unwrap().filter_map(Result::ok).collect();
    res
}

