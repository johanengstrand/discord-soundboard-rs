use crate::bot;
use crate::CONFIG;
use crate::api::Handles;

use serenity::http::client::Http;
use serenity::model::id::ChannelId;

pub async fn tracks() -> Result<impl warp::Reply, warp::Rejection> {
    let track_list = bot::tracks::get_tracks_in_dir(&CONFIG.folder);

    match track_list {
        Ok(tracks) => success!(tracks),
        Err(why) => failure!(format!("Failed to get tracks: {}", why)),
    }
}

// pub async fn test2(http: Arc<Http>) -> Result<impl warp::Reply, warp::Rejection> {
//     let map = serde_json::json!({"content": "Hello from API"});

//     match http.send_message(220604818685689856 as u64, &map).await {
//         Ok(_) => success!("Sent message"),
//         Err(why) => failure!(format!("Failed to send message: {}", why)),
//     }
// }
