use crate::api;
use crate::bot;
use crate::CONFIG;

use std::sync::{Arc, Mutex};
use songbird::Songbird;
use serenity::CacheAndHttp;
use serenity::Client;
use serenity::http::client::Http;
use serenity::model::id::ChannelId;

pub async fn join(ctx: Arc<CacheAndHttp>, manager: Arc<Songbird>)
    -> Result<impl warp::Reply, warp::Rejection> {
    match bot::guilds::get_current_voice_channel(ctx, CONFIG.user_id).await {
        Ok(Some(data)) => {
            let (_, join_result) = manager.join(data.guild_id, data.channel_id).await;
            match join_result {
                Ok(_) => success!("Joined channel!"),
                Err(why) => failure!(format!("Could not join channel ({})", why)),

            }
        },
        Ok(None) => failure!(format!("You are not connected to a voice channel")),
        Err(why) => failure!(format!("Could not join channel: {}", why)),
    }

}
pub async fn tracks() -> Result<impl warp::Reply, warp::Rejection> {
    let track_list = bot::tracks::get_tracks_in_dir(&CONFIG.folder);

    match track_list {
        Ok(tracks) => success!(tracks),
        Err(why) => failure!(format!("Failed to get tracks: {}", why)),
    }
}

pub async fn play(track: String) -> Result<impl warp::Reply, warp::Rejection> {
    // let play_track = bot::playback::play(track);

    success!(track)

    // match play_track {
    //     Ok(play) => success!(play),
    //     Err(why) => failure!(format!("Failed to play track: {}", why)),
    // }
}

pub async fn stop() -> Result<impl warp::Reply, warp::Rejection> {
    // let stop_track = bot::playback::stop(&CONFIG.folder);

    // match stop_track {
    //     Ok(stop) => success!(stop),
    //     Err(why) => failure!(format!("Failed to stop track: {}", why)),
    // }
    success!("Not implemented")
}

pub async fn favorite(track: String) -> Result<impl warp::Reply, warp::Rejection> {
    let track_list = bot::tracks::get_tracks_in_dir(&CONFIG.folder);

    match track_list {
        Ok(_) => success!(format!("Favorited track: {}", track)),
        Err(why) => failure!(format!("Failed to add to favorites: {}", why)),
    }
}

pub async fn unfavorite(track: String) -> Result<impl warp::Reply, warp::Rejection> {
    let track_list = bot::tracks::get_tracks_in_dir(&CONFIG.folder);

    match track_list {
        Ok(_) => success!(format!("Unfavorited track: {}", track)),
        Err(why) => failure!(format!("Failed to remove from favorites: {}", why)),
    }
}
// pub async fn test2(http: Arc<Http>) -> Result<impl warp::Reply, warp::Rejection> {
//     let map = serde_json::json!({"content": "Hello from API"});

//     match http.send_message(220604818685689856 as u64, &map).await {
//         Ok(_) => success!("Sent message"),
//         Err(why) => failure!(format!("Failed to send message: {}", why)),
//     }
// }
