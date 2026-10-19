use std::io;
use std::process::ExitCode;

use rusqlite::Connection;

fn io_envkey2val(key: &'static str) -> impl Fn() -> String {
    move || std::env::var(key).unwrap_or_default()
}

fn io_path2sqlite() -> impl Fn() -> String {
    io_envkey2val("ENV_SQLITE_DB_FILE_PATH")
}

fn io_conn() -> impl FnMut() -> Result<Connection, io::Error> {
    || {
        let path2sqlite: String = io_path2sqlite()();
        if path2sqlite.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "empty path got",
            ));
        }
        rs_kvpairs2sqlite::path2con(path2sqlite)
    }
}

fn io_with_conn() -> impl FnMut(Connection) -> Result<(), io::Error> {
    |mut conn: Connection| {
        rs_kvpairs2sqlite::stdin2kvpairs2sqlite(&mut conn)?;
        Ok(())
    }
}

fn io_main() -> impl FnOnce() -> Result<(), io::Error> {
    || {
        let conn: Connection = io_conn()()?;
        io_with_conn()(conn)?;
        Ok(())
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        let path2sqlite: String = io_path2sqlite()();
        eprintln!("ENV_SQLITE_DB_FILE_PATH: {path2sqlite}");
        ExitCode::FAILURE
    })
}
