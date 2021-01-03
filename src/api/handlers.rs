use crate::bot;
use crate::CONFIG;

use std::sync::Arc;
use tokio::sync::Mutex;
use songbird::Songbird;
use serenity::CacheAndHttp;

pub async fn join(ctx: Arc<CacheAndHttp>, manager: Arc<Songbird>, bot_state: Arc<Mutex<bot::State>>)
    -> Result<impl warp::Reply, warp::Rejection> {
    match bot::guilds::get_current_voice_channel(ctx).await {
        Ok(Some(data)) => {
            let (call_handle, join_result) = manager.join(data.guild_id, data.channel_id).await;
            match join_result {
                Ok(_) => {
                    let mut bot_state_lock = bot_state.lock().await;
                    bot_state_lock.current_guild_id = Some(data.guild_id);
                    bot_state_lock.current_call = Some(call_handle.clone());
                    return success!("Joined channel!")
                },
                Err(why) => failure!(format!("Could not join channel ({})", why)),

            }
        },
        Ok(None) => failure!(format!("You are not connected to a voice channel")),
        Err(why) => failure!(format!("Could not join channel: {}", why)),
    }
}

pub async fn leave(manager: Arc<Songbird>, bot_state: Arc<Mutex<bot::State>>)
    -> Result<impl warp::Reply, warp::Rejection> {
    let mut bot_state_lock = bot_state.lock().await;
    match bot_state_lock.current_guild_id {
        Some(guild_id) => {
            match manager.leave(guild_id).await {
                Ok(_) => {
                    bot_state_lock.current_guild_id = None;
                    bot_state_lock.current_call = None;
                    bot::playback::stop_all(&mut bot_state_lock.cached_tracks).await;
                    success!("Left channel")
                },
                Err(why) => failure!(format!("Could not leave channel ({})", why)),
            }
        },
        None => failure!("The bot is not currently connected to a voice channel"),
    }
}

pub async fn connected(bot_state: Arc<Mutex<bot::State>>)
    -> Result<impl warp::Reply, warp::Rejection> {
    match bot_state.lock().await.current_call {
        Some(_) => success!(true),
        None => success!(false),
    }
}

// TODO: If bot state contains non-empty hash map of tracks, return it instead
// TODO: Add route for reloading the loaded tracks
pub async fn tracks() -> Result<impl warp::Reply, warp::Rejection> {
    let track_list = bot::tracks::get_tracks_in_dir(&CONFIG.folder);

    match track_list {
        Ok(mut tracks) => {
            bot::collections::add_collections_to_tracks(&mut tracks);
            success!(tracks)
        },
        Err(why) => failure!(format!("Failed to get tracks: {}", why)),
    }
}

pub async fn play(bot_state: Arc<Mutex<bot::State>>, path: String)
    -> Result<impl warp::Reply, warp::Rejection> {
    let mut bot_state_lock = bot_state.lock().await;

    match bot::playback::play(&mut bot_state_lock, &path).await {
        // serde_json does not support u128? Casting to u64 works
        Ok(duration) => success!(duration as u64),
        Err(why) => failure!(format!("Failed to play track: {}", why)),
    }
}

pub async fn stop(bot_state: Arc<Mutex<bot::State>>, path: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut bot_state_lock = bot_state.lock().await;

    match bot::playback::stop(&mut bot_state_lock.cached_tracks, &path).await {
        Ok(_) => success!(format!("Stopped track: {}", path)),
        Err(why) => failure!(format!("Failed to stop track: {}", why)),
    }
}

pub async fn favorite(path: String) -> Result<impl warp::Reply, warp::Rejection> {
    success!(format!("Favorited track: {}", path))
    // match track_list {
    //     Ok(_) => success!(format!("Favorited track: {}", path)),
    //     Err(why) => failure!(format!("Failed to add to favorites: {}", why)),
    // }
}

pub async fn unfavorite(path: String) -> Result<impl warp::Reply, warp::Rejection> {
    success!(format!("Favorited track: {}", path))
    // match track_list {
    //     Ok(_) => success!(format!("Unfavorited track: {}", path)),
    //     Err(why) => failure!(format!("Failed to remove from favorites: {}", why)),
    // }
}
