use std::io;
use std::path::Path;

use io::BufRead;

use rusqlite::Connection;

use rusqlite::Statement;
use rusqlite::Transaction;

pub fn path2con<P>(path2sqlite_file: P) -> Result<Connection, io::Error>
where
    P: AsRef<Path>,
{
    Connection::open(path2sqlite_file).map_err(io::Error::other)
}

pub fn print_total_changes(c: &Connection) -> Result<(), io::Error> {
    let changes: u64 = c.total_changes();
    println!("total changes: {changes}");
    Ok(())
}

pub fn brdr2kvpairs<R>(brdr: R) -> impl Iterator<Item = Result<(String, String), io::Error>>
where
    R: BufRead,
{
    serde_json::Deserializer::from_reader(brdr)
        .into_iter()
        .map(|r| r.map_err(io::Error::other))
}

pub fn kvpairs2sqlite<I>(kvpairs: I, conn: &mut Connection) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<(String, String), io::Error>>,
{
    conn.execute(
        r#"
            CREATE TABLE IF NOT EXISTS kvpairs (
                key TEXT PRIMARY KEY,
                val TEXT
            )
            WITHOUT ROWID
            ;
        "#,
        (),
    )
    .map_err(io::Error::other)?;

    let tx: Transaction = conn.transaction().map_err(io::Error::other)?;

    let mut stmt: Statement = tx
        .prepare(
            r#"
                INSERT INTO kvpairs
                VALUES (?1, ?2)
                ON CONFLICT(key) DO NOTHING
            "#,
        )
        .map_err(io::Error::other)?;

    for rpair in kvpairs {
        let pair: (_, _) = rpair?;
        let (key, val) = pair;
        stmt.execute([key, val]).map_err(io::Error::other)?;
    }

    stmt.finalize().map_err(io::Error::other)?;
    tx.commit().map_err(io::Error::other)?;

    Ok(())
}

pub fn brdr2kvpairs2sqlite<R>(brdr: R, conn: &mut Connection) -> Result<(), io::Error>
where
    R: BufRead,
{
    kvpairs2sqlite(brdr2kvpairs(brdr), conn)
}

pub fn stdin2kvpairs2sqlite(conn: &mut Connection) -> Result<(), io::Error> {
    brdr2kvpairs2sqlite(io::stdin().lock(), conn)
}
