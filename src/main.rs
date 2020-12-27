#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::io;
use std::env;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use rocket_contrib::serve::StaticFiles;

mod api;
mod discord;

#[derive(Debug)]
struct Track {
    name: PathBuf,
}

fn get_tracks_in_dir(dir: &str) -> Result<Vec<Track>, String> {
    let path = PathBuf::from(dir);
    let mut tracks = Vec::new();

    match fs::read_dir(path) {
        Err(why) => println!("error"),
        Ok(paths) => for path in paths {
            tracks.push(Track { name: path.unwrap().path() });
        },
    }

    Ok(tracks)
}

fn start_webserver() {
    api::routes::mount()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))
        .launch();
}

fn main() {
    // TODO: Error handling
    // Configure the client with your Discord bot token in the environment.
    let track_list = get_tracks_in_dir("/home/johan/nextcloud/VERY_IMPORTANT_FILES");
    println!("{:?}", track_list.unwrap());
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    discord::bot::start(token);
    start_webserver();
}
