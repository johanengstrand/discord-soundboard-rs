use crate::bot;
use crate::CONFIG;

use std::sync::Arc;
use tokio::sync::RwLock;
use songbird::Songbird;
use serenity::CacheAndHttp;

pub async fn join(ctx: Arc<CacheAndHttp>, manager: Arc<Songbird>, bot_state: Arc<RwLock<bot::State>>)
    -> Result<impl warp::Reply, warp::Rejection> {
    match bot::guilds::get_current_voice_channel(ctx).await {
        Ok(Some(data)) => {
            let (call_handle, join_result) = manager.join(data.guild_id, data.channel_id).await;
            match join_result {
                Ok(_) => {
                    let mut bot_state_lock = bot_state.write().await;
                    bot_state_lock.connected_guild_id = Some(data.guild_id);
                    bot_state_lock.current_call = Some(call_handle);
                    return success!("Joined channel!")
                },
                Err(why) => failure!(format!("Could not join channel ({})", why)),

            }
        },
        Ok(None) => failure!(format!("You are not connected to a voice channel")),
        Err(why) => failure!(format!("Could not join channel: {}", why)),
    }
}

pub async fn leave(manager: Arc<Songbird>, bot_state: Arc<RwLock<bot::State>>)
    -> Result<impl warp::Reply, warp::Rejection> {
    match bot_state.read().await.connected_guild_id {
        Some(guild_id) => {
            match manager.leave(guild_id).await {
                // TODO: Reset bot state
                Ok(_) => success!("Left channel"),
                Err(why) => failure!(format!("Could not leave channel ({})", why)),
            }
        },
        None => failure!("The bot is not currently connected to a voice channel"),
    }
}

pub async fn tracks() -> Result<impl warp::Reply, warp::Rejection> {
    let track_list = bot::tracks::get_tracks_in_dir(&CONFIG.folder);

    match track_list {
        Ok(tracks) => success!(tracks),
        Err(why) => failure!(format!("Failed to get tracks: {}", why)),
    }
}

pub async fn play(manager: Arc<Songbird>, bot_state: Arc<RwLock<bot::State>>, track: String)
    -> Result<impl warp::Reply, warp::Rejection> {
    match &bot_state.write().await.current_call {
        Some(call_lock) => {
            match bot::playback::play(call_lock.clone(), &track).await {
                Ok(_) => success!(format!("Played track {}", track)),
                Err(why) => failure!(format!("Failed to play track: {}", why)),
            }
        },
        None => failure!("The bot is not currently connected to a voice channel"),
    }
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
