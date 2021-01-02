#![feature(proc_macro_hygiene, decl_macro)]

extern crate warp;
extern crate serde_json;
#[macro_use] extern crate serde;
#[macro_use] extern crate lazy_static;

use std::process;
use warp::Filter;
use songbird::Songbird;
use serenity::client::Client;

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

fn json_body() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn start() {
    let songbird = Songbird::serenity();
    let mut client = songbird::serenity::register_with(Client::builder(&CONFIG.token), songbird.clone())
        .await
        .expect("Failed to create discord client");

    let ctx = client.cache_and_http.clone();
    let bot_state = bot::State::new();

    tokio::spawn(async move {
        let ctx_filter = warp::any().map(move || ctx.clone());
        let songbird_filter = warp::any().map(move || songbird.clone());
        let bot_state_filter = warp::any().map(move || bot_state.clone());

        let join = warp::post()
            .and(warp::path("api"))
            .and(warp::path("join"))
            .and(warp::path::end())
            .and(ctx_filter.clone())
            .and(songbird_filter.clone())
            .and(bot_state_filter.clone())
            .and_then(api::routes::join);

        let leave = warp::post()
            .and(warp::path("api"))
            .and(warp::path("leave"))
            .and(warp::path::end())
            .and(songbird_filter.clone())
            .and(bot_state_filter.clone())
            .and_then(api::routes::leave);

        let connected = warp::get()
            .and(warp::path("api"))
            .and(warp::path("connected"))
            .and(warp::path::end())
            .and(bot_state_filter.clone())
            .and_then(api::routes::connected);

        let tracks = warp::get()
            .and(warp::path("api"))
            .and(warp::path("tracks"))
            .and(warp::path::end())
            .and_then(api::routes::tracks);

        let play = warp::post()
            .and(warp::path("api"))
            .and(warp::path("play"))
            .and(warp::path::end())
            .and(bot_state_filter.clone())
            .and(json_body())
            .and_then(api::routes::play);

        let stop = warp::post()
            .and(warp::path("api"))
            .and(warp::path("stop"))
            .and(warp::path::end())
            .and(bot_state_filter.clone())
            .and(json_body())
            .and_then(api::routes::stop);

        let favorite = warp::post()
            .and(warp::path("api"))
            .and(warp::path("favorite"))
            .and(warp::path::end())
            .and(json_body())
            .and_then(api::routes::favorite);

        let unfavorite = warp::post()
            .and(warp::path("api"))
            .and(warp::path("unfavorite"))
            .and(warp::path::end())
            .and(json_body())
            .and_then(api::routes::unfavorite);

        let public = warp::fs::dir("public");

        let routes = public
            .or(join)
            .or(leave)
            .or(connected)
            .or(play)
            .or(stop)
            .or(tracks)
            .or(favorite)
            .or(unfavorite);

        warp::serve(routes)
            .run(([0, 0, 0, 0], 8000))
            .await;
    });

    let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
}

fn main() {
    start();
}
