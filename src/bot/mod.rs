use songbird::Call;
use tokio::sync::Mutex;
use crate::bot::playback::CachedTrack;

use std::{
    sync::Arc,
    collections::HashMap,
};

pub mod tracks;
pub mod guilds;
pub mod playback;

pub struct State {
    pub current_guild_id: Option<u64>,
    pub current_call: Option<Arc<Mutex<Call>>>,
    pub cached_tracks: HashMap<String, CachedTrack>,
}

impl State {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            State {
                current_guild_id: None,
                current_call: None,
                cached_tracks: HashMap::new(),
            }
        ))
    }
}
