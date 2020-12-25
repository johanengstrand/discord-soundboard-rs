#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::env;
use rocket_contrib::serve::StaticFiles;

mod api;
mod discord;

fn start_webserver() {
    api::routes::mount()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))
        .launch();
}

fn main() {
    // TODO: Error handling
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    discord::bot::start(token);
    start_webserver();
}
