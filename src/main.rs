#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::env;
use std::thread;

mod api;
mod bot;

fn main() {
    // TODO: Error handling
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let api_thread = thread::spawn(||api::start());

    bot::start(token);
    api_thread.join().unwrap();
}
