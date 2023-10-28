use rusqlite::{params, Connection, Result};
use rust_sqlite_project::{delete_track, insert_track, setup_db, update_track_bpm};

const DB_FILE: &str = "spotify_2023_1_new_test.db";

// Check if the database file exists.
fn db_exists() -> bool {
    std::fs::metadata(DB_FILE).is_ok()
}

#[test]
fn test_setup_db() -> Result<()> {
    // Check if the DB exists, and delete it if so
    if db_exists() {
        if let Err(e) = std::fs::remove_file(DB_FILE) {
            if e.kind() != std::io::ErrorKind::NotFound {
                panic!("Failed to remove test database: {:?}", e);
            }
        }
    }
    // Set up the database.
    setup_db()?;
    assert!(db_exists());

    Ok(())
}

#[test]
fn test_insert_track() -> Result<()> {
    if db_exists() {
        std::fs::remove_file(DB_FILE).expect("Failed to remove test database");
    }

    setup_db()?;

    let conn = Connection::open(DB_FILE)?;

    // Check the number of rows before the insert
    let count_before: i32 =
        conn.query_row("SELECT COUNT(*) FROM tracks", params![], |r| r.get(0))?;
    println!("Number of rows before insert: {}", count_before);

    insert_track("Artist Test", 10, 5, 120)?;

    // Check the number of rows after the insert
    let count_after: i32 = conn.query_row(
        "SELECT COUNT(*) FROM tracks WHERE artist_name = 'Artist Test'",
        params![],
        |r| r.get(0),
    )?;
    assert_eq!(count_after, count_before + 1);

    Ok(())
}

#[test]
fn test_update_track_bpm() -> Result<()> {
    setup_db()?;
    insert_track("Artist Test", 10, 5, 120)?;
    update_track_bpm("Artist Test", 130)?;

    let conn = Connection::open(DB_FILE)?;
    let bpm: i32 = conn.query_row(
        "SELECT bpm FROM tracks WHERE artist_name = 'Artist Test'",
        params![],
        |r| r.get(0),
    )?;
    assert_eq!(bpm, 130);

    Ok(())
}

#[test]
fn test_delete_track() -> Result<()> {
    setup_db()?;
    insert_track("Artist Test", 10, 5, 120)?;
    delete_track("Artist Test")?;

    let conn = Connection::open(DB_FILE)?;
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM tracks WHERE artist_name = 'Artist Test'",
        params![],
        |r| r.get(0),
    )?;
    assert_eq!(count, 0);

    Ok(())
}
