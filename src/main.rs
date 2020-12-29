#![feature(proc_macro_hygiene, decl_macro)]
extern crate serde_json;
#[macro_use] extern crate warp;
#[macro_use] extern crate serde;
#[macro_use] extern crate lazy_static;

use std::sync::Arc;
use std::process;
use warp::Filter;
use songbird::SerenityInit;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{gateway::Ready},
    http::{client::Http},
};

mod api;
mod bot;
mod settings;

lazy_static! {
    static ref CONFIG: settings::Settings = match settings::Settings::new() {
        Err(why) => {
            println!("Failed to parse configuration file ({})", why);
            process::exit(1);
        },
        Ok(settings) => settings,
    };
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn start() {
    let mut client = Client::builder(&CONFIG.token)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Failed to create discord client!");

    let http = Arc::new(Http::new_with_token(&CONFIG.token));

    tokio::spawn(async move {
        let http_filter = warp::any().map(move || http.clone());
        let tracks = warp::get()
            .and(warp::path("api"))
            .and(warp::path("tracks"))
            .and(warp::path::end())
            .and_then(api::routes::tracks);

        let public = warp::fs::dir("public");
        let routes = tracks.or(public);

        warp::serve(routes)
            .run(([0, 0, 0, 0], 8000))
            .await;
    });

    let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
}

fn main() {
    start();
}
