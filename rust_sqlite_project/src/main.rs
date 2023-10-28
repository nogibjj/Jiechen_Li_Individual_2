// fn main() {
//     println!("Hello, world!");
// }
extern crate rust_sqlite_project; 
use rust_sqlite_project::{setup_db, insert_track, query_tracks, update_track_bpm, delete_track};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "setup" => {
            if let Err(err) = setup_db() {
                eprintln!("Error: {:?}", err);
            }
        }
        "insert" => {
            if args.len() >= 6 {
                let artist_name = &args[2];
                let spotify: i32 = args[3].parse().unwrap_or(0);
                let apple: i32 = args[4].parse().unwrap_or(0);
                let bpm: i32 = args[5].parse().unwrap_or(0);
                
                if let Err(err) = insert_track(artist_name, spotify, apple, bpm) {
                    eprintln!("Error: {:?}", err);
                }
            } else {
                println!("Usage: {} insert [artist_name] [spotify] [apple] [bpm]", args[0]);
            }
        }
        "query" => {
            if let Err(err) = query_tracks() {
                eprintln!("Error: {:?}", err);
            }
        }
        "update" => {
            if args.len() >= 4 {
                let artist_name = &args[2];
                let new_bpm: i32 = args[3].parse().unwrap_or(0);
                
                if let Err(err) = update_track_bpm(artist_name, new_bpm) {
                    eprintln!("Error: {:?}", err);
                }
            } else {
                println!("Usage: {} update [artist_name] [new_bpm]", args[0]);
            }
        }
        "delete" => {
            if args.len() >= 3 {
                let artist_name = &args[2];
                
                if let Err(err) = delete_track(artist_name) {
                    eprintln!("Error: {:?}", err);
                }
            } else {
                println!("Usage: {} delete [artist_name]", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'setup', 'insert', 'query', 'update', or 'delete'.");
        }
    }
}