use songbird::{
    Call,
    tracks::TrackHandle,
};
use std::{
    sync::Arc,
    collections::HashMap,
};
use tokio::sync::Mutex;

pub mod tracks;
pub mod guilds;
pub mod playback;

pub struct State {
    pub current_guild_id: Option<u64>,
    pub current_call: Option<Arc<Mutex<Call>>>,
    pub current_tracks: HashMap<String, TrackHandle>,
}

impl State {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            State {
                current_guild_id: None,
                current_call: None,
                current_tracks: HashMap::new(),
            }
        ))
    }
}
