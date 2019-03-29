
use rusqlite::{Connection, NO_PARAMS, OpenFlags};

macro_rules! DB_DEFAULTS {
    (DB_PATH) => ("./benchy.db");
    (DB_TBL) => (
        "CREATE TABLE submissions (
            sub_id SERIAL PRIMARY KEY,
            ident VARCHAR(32),
            data TEXT
        );"
    )
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

