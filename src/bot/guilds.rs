use crate::CONFIG;
use std::{
    sync::Arc,
    collections::HashMap,
};

use serenity::{
    CacheAndHttp,
    http::GuildPagination,
    prelude::SerenityError,
    client::Cache,
    model::{
        channel::{
            ChannelType,
            GuildChannel,
        },
        id::{
            GuildId,
            ChannelId,
        },
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentUserChannel {
    pub guild_id: u64,
    pub channel_id: u64,
}

impl CurrentUserChannel {
    pub fn new(guild_id: GuildId, channel_id: ChannelId) -> Self {
        CurrentUserChannel {
            guild_id: guild_id.as_u64().clone(),
            channel_id: channel_id.as_u64().clone(),
        }
    }
}

/// Finds a user in a hash map of channels by user id.
/// Only searches through channels whose type is `ChannelType::Voice`.
async fn find_user_in_voice_channel(
    channels: HashMap<ChannelId,
    GuildChannel>,
    user_id: u64,
    cache: Arc<Cache>
) -> Result<Option<ChannelId>, SerenityError> {
    for (_, channel) in channels.iter() {
        match channel.kind {
            ChannelType::Voice => {
                for member in channel.members(cache.clone()).await?.iter() {
                    if member.user.id == user_id {
                        return Ok(Some(channel.id));
                    }
                }
            },
            _ => continue,
        }
    }

    Ok(None)
}

pub async fn get_current_voice_channel(ctx: Arc<CacheAndHttp>)
    -> Result<Option<CurrentUserChannel>, SerenityError> {
    let guilds = ctx.http.get_guilds(&GuildPagination::After(GuildId(0)), 100 as u64).await?;

    for guild in guilds.iter() {
        match guild.id.channels(ctx.http.clone()).await {
            Ok(channels) => {
                match find_user_in_voice_channel(channels, CONFIG.user_id, ctx.cache.clone()).await {
                    Ok(None) => continue,
                    Ok(Some(channel_id)) => return Ok(Some(CurrentUserChannel::new(guild.id, channel_id))),
                    Err(why) => return Err(why),
                }
            },
            Err(_) => break,
        }

    }

    Ok(None)
}
