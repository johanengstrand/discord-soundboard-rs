use crate::CONFIG;
use std::sync::Arc;
use serenity::{
    CacheAndHttp,
    Client,
    prelude::SerenityError,
    http::{client::Http, GuildPagination},
    model::{
        channel::{ChannelType, GuildChannel, Message},
        guild::GuildInfo,
        id::GuildId,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentUserChannel {
    pub guild_id: u64,
    pub channel_id: u64,
}

pub async fn get_current_voice_channel(ctx: Arc<CacheAndHttp>, user_id: u64)
    -> Result<Option<CurrentUserChannel>, SerenityError> {
    let guilds = ctx.http.get_guilds(&GuildPagination::After(GuildId(0)), 100 as u64).await?;

    for guild in guilds.iter() {
        let channels = ctx.http.get_channels(*guild.id.as_u64()).await?;
        for channel in channels.iter() {
            match channel.kind {
                ChannelType::Voice =>
                {
                    for member in channel.members(ctx.cache.clone()).await?.iter() {
                        if member.user.id == CONFIG.user_id {
                            return Ok(Some(CurrentUserChannel {
                                guild_id: guild.id.as_u64().clone(),
                                channel_id: channel.id.as_u64().clone(),
                            }))
                        }
                    }
                },
                _ => continue,
            }
        }
    }

    Ok(None)
}
