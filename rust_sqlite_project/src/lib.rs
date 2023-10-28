use rusqlite::{params, Connection, Result};
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(LOG_FILE) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn setup_db() -> Result<()> {
    let conn = Connection::open("spotify_2023_1_new_test.db")?;

    conn.execute("DROP TABLE IF EXISTS tracks", [])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            artist_name TEXT NOT NULL,
            in_spotify_playlists INTEGER,
            in_apple_playlists INTEGER,
            bpm INTEGER
        )",
        [],
    )?;

    Ok(())
}

pub fn insert_track(artist_name: &str, spotify: i32, apple: i32, bpm: i32) -> Result<()> {
    let conn = Connection::open("spotify_2023_1_new_test.db")?;
    conn.execute(
        "INSERT INTO tracks (artist_name, in_spotify_playlists, in_apple_playlists, bpm) VALUES (?1, ?2, ?3, ?4)",
        params![artist_name, spotify, apple, bpm],
    )?;
    log_query(
        &format!(
            "INSERT INTO tracks (artist_name, in_spotify_playlists, in_apple_playlists, bpm) VALUES ('{}', {}, {}, {});",
            artist_name, spotify, apple, bpm
        )
    );

    Ok(())
}

pub fn query_tracks() -> Result<()> {
    let conn = Connection::open("spotify_2023_1_new_test.db")?;
    let mut stmt = conn.prepare("SELECT artist_name, in_spotify_playlists, in_apple_playlists, bpm FROM tracks")?;
    let tracks = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, String>(0)?,
            row.get::<usize, i32>(1)?,
            row.get::<usize, i32>(2)?,
            row.get::<usize, i32>(3)?,
        ))
    })?;

    for track in tracks {
        match track {
            Ok((artist_name, spotify, apple, bpm)) => {
                println!("Result: artist_name={}, in_spotify_playlists={}, in_apple_playlists={}, bpm={}", artist_name, spotify, apple, bpm);
            }
            Err(e) => eprintln!("Error in row: {:?}", e),
        }
    }

    log_query("SELECT artist_name, in_spotify_playlists, in_apple_playlists, bpm FROM tracks;");

    Ok(())
}

pub fn update_track_bpm(artist_name: &str, new_bpm: i32) -> Result<()> {
    let conn = Connection::open("spotify_2023_1_new_test.db")?;
    conn.execute(
        "UPDATE tracks SET bpm = ?1 WHERE artist_name = ?2",
        params![new_bpm, artist_name],
    )?;

    log_query(
        &format!(
            "UPDATE tracks SET bpm = {} WHERE artist_name = '{}';",
            new_bpm, artist_name
        )
    );

    Ok(())
}

pub fn delete_track(artist_name: &str) -> Result<()> {
    let conn = Connection::open("spotify_2023_1_new_test.db")?;
    conn.execute("DELETE FROM tracks WHERE artist_name = ?", params![artist_name])?;
    log_query(&format!("DELETE FROM tracks WHERE artist_name = '{}';", artist_name));

    Ok(())
}

