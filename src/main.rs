#![feature(proc_macro_hygiene, decl_macro)]

extern crate warp;
extern crate serde_json;
#[macro_use] extern crate serde;
#[macro_use] extern crate lazy_static;

use std::process;
use songbird::Songbird;
use serenity::client::Client;

mod api;
mod bot;
mod settings;

lazy_static! {
    /// Global configuration store.
    /// Contains the data read from `config.json` and can be imported
    /// and used without any initilization since it is static.
    ///
    /// # Example
    /// ```rust
    /// use crate::CONFIG;
    /// ...
    /// let track_list = bot::tracks::get_tracks_in_dir(&CONFIG.folder);
    /// ```
    static ref CONFIG: settings::Settings = match settings::Settings::new() {
        Err(why) => {
            println!("Failed to parse configuration file ({})", why);
            process::exit(1);
        },
        Ok(settings) => settings,
    };
}

#[tokio::main]
async fn start() {
    // Create a new instance of Songbird to be used with serenity and saved into a filter
    let bot_state = bot::State::new();
    let songbird = Songbird::serenity();

    let mut client = songbird::serenity::register_with(
        Client::builder(&CONFIG.token),
        songbird.clone()
    ).await.expect("Failed to create discord client");

    // The HTTP client is used to find the current voice channel of the user
    let ctx = client.cache_and_http.clone();

    tokio::spawn(async move {
        let routes = api::routes::create(
            bot_state.clone(),
            songbird.clone(),
            ctx.clone()
        );

        warp::serve(routes)
            .run(([0, 0, 0, 0], 8000))
            .await;
    });

    let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
}

fn main() {
    start();
}
