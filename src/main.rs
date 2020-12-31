#![feature(proc_macro_hygiene, decl_macro)]
extern crate serde_json;
#[macro_use] extern crate warp;
#[macro_use] extern crate serde;
#[macro_use] extern crate lazy_static;

use std::{collections::HashMap, convert::TryInto, env, sync::Arc};
use std::process;
use warp::Filter;
use songbird::{
    input::{
        self,
        cached::{Compressed, Memory},
        Input,
    },
    Bitrate,
    Call,
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
    SerenityInit,
    TrackEvent,
    Songbird,
};
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    http::{client::Http},
    model::{channel::Message, gateway::Ready, misc::Mentionable},
    prelude::{Mutex, TypeMapKey},
    Result as SerenityResult,
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

struct SoundStore;

enum CachedSound {
    Compressed(Compressed),
    Uncompressed(Memory),
}

impl TypeMapKey for SoundStore {
    type Value = Arc<Mutex<HashMap<String, CachedSound>>>;
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

    // {
    //     let mut data = client.data.write().await;

    //     // Loading the audio ahead of time.
    //     let mut audio_map = HashMap::new();

    //     // let ting_src = Memory::new(
    //     //     input::ffmpeg("ting.wav").await.expect("File should be in root folder."),
    //     // ).expect("These parameters are well-defined.");
    //     // let _ = ting_src.raw.spawn_loader();
    //     // audio_map.insert("ting".into(), CachedSound::Uncompressed(ting_src));

    //     data.insert::<SoundStore>(Arc::new(Mutex::new(audio_map)));
    // }

    tokio::spawn(async move {
        let ctx_filter = warp::any().map(move || ctx.clone());
        let songbird_filter = warp::any().map(move || songbird.clone());

        let join = warp::get()
            .and(warp::path("api"))
            .and(warp::path("join"))
            .and(warp::path::end())
            .and(ctx_filter.clone())
            .and(songbird_filter.clone())
            .and_then(api::routes::join);

        let tracks = warp::get()
            .and(warp::path("api"))
            .and(warp::path("tracks"))
            .and(warp::path::end())
            .and_then(api::routes::tracks);

        let play = warp::post()
            .and(warp::path("api"))
            .and(warp::path("play"))
            .and(warp::path::end())
            .and(json_body())
            .and_then(api::routes::play);

        let stop = warp::post()
            .and(warp::path("api"))
            .and(warp::path("stop"))
            .and(warp::path::end())
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

        let routes = tracks.or(public).or(join).or(play).or(stop).or(favorite).or(unfavorite);

        warp::serve(routes)
            .run(([0, 0, 0, 0], 8000))
            .await;
    });

    let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
}

fn main() {
    start();
}
