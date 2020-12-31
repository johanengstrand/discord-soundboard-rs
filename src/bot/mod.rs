pub mod tracks;
pub mod guilds;
pub mod playback;

pub struct State {
    pub connected_guild_id: Option<u64>,
}
