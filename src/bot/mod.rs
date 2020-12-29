use songbird::SerenityInit;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{gateway::Ready},
    http::{client::Http},
};

pub mod tracks;
pub mod guilds;
pub mod playback;
