use songbird::Call;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod tracks;
pub mod guilds;
pub mod playback;

pub struct State {
    pub connected_guild_id: Option<u64>,
    pub current_call: Option<Arc<Mutex<Call>>>,
}
