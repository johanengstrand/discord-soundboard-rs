use crate::bot;
use crate::api;
use std::sync::Arc;
use warp::Filter;
use tokio::sync::Mutex;
use songbird::Songbird;
use serenity::CacheAndHttp;

/// Warp filter for extracting JSON data from requests.
/// This filter extracts a `String` and will return an invalid status code
/// if the request body is invalid.
///
/// # Example
/// ```rust
/// let route = warp::post()
///     .and(warp::path("api"))
///     .and(warp::path("endpoint"))
///     .and(warp::path::end())
///     .and(json_body())
///     .and_then(handle_route);
/// ...
/// // The function `handle_route` will now take one parameter of type `String`
/// pub async fn handle_route(data: String) {...}
/// ```
fn json_body() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Creates a filter containing all available routes and registers the
/// data filters and paths for each route handler.
///
/// # Arguments
/// * `bot_state` - The empty bot state
/// * `songbird` - The Songbird instance registered to serenity
/// * `ctx` - The CacheAndHttp instance of the created serenity client
///
/// # Example
/// ```rust
/// let routes = api::routes::create(
///     bot_state.clone(),
///     songbird.clone(),
///     ctx.clone()
/// );
///
/// warp::serve(routes)
///     .run(([0, 0, 0, 0], 8000))
///     .await;
///
/// ```
pub fn create(
    bot_state: Arc<Mutex<bot::State>>,
    songbird: Arc<Songbird>,
    ctx: Arc<CacheAndHttp>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let ctx_filter = warp::any().map(move || ctx.clone());
    let songbird_filter = warp::any().map(move || songbird.clone());
    let bot_state_filter = warp::any().map(move || bot_state.clone());

    let join = warp::post()
        .and(warp::path!("api" / "join"))
        .and(ctx_filter.clone())
        .and(songbird_filter.clone())
        .and(bot_state_filter.clone())
        .and_then(api::handlers::join);

    let leave = warp::post()
        .and(warp::path!("api" / "leave"))
        .and(songbird_filter.clone())
        .and(bot_state_filter.clone())
        .and_then(api::handlers::leave);

    let connected = warp::get()
        .and(warp::path!("api" / "connected"))
        .and(bot_state_filter.clone())
        .and_then(api::handlers::connected);

    let tracks = warp::get()
        .and(warp::path!("api" / "tra"))
        .and_then(api::handlers::tracks);

    let play = warp::post()
        .and(warp::path!("api" / "play"))
        .and(bot_state_filter.clone())
        .and(json_body())
        .and_then(api::handlers::play);

    let stop = warp::post()
        .and(warp::path!("api" / "stop"))
        .and(bot_state_filter.clone())
        .and(json_body())
        .and_then(api::handlers::stop);

    let favorite = warp::post()
        .and(warp::path!("api" / "favorite"))
        .and(json_body())
        .and_then(api::handlers::favorite);

    let unfavorite = warp::post()
        .and(warp::path!("api" / "unfavorite"))
        .and(json_body())
        .and_then(api::handlers::unfavorite);

    let public = warp::fs::dir("public");

    public
        .or(join)
        .or(leave)
        .or(connected)
        .or(play)
        .or(stop)
        .or(tracks)
        .or(favorite)
        .or(unfavorite)
}
