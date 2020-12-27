use serenity::{
    http::{client::Http, GuildPagination},
    model::{
        channel::{ChannelType, GuildChannel, Message},
        guild::GuildInfo,
        id::GuildId,
    },
};

// pub async fn get_current_voice_channel(client: &Http, user_id: String) -> {
//     let guilds = client.get_guilds(&GuildPagination::After(GuildId(0u64)), 100).await;
// }

// async fn get_all_guilds(client: &Http) -> Result<Vec<GuildInfo>> {
//     let mut last_guild_id = Some(0u64);
//     let mut guilds: Vec<GuildInfo> = vec![];
//     while let Some(after) = last_guild_id {
//         let mut batch = client
//             .get_guilds(&GuildPagination::After(GuildId(after)), 100)
//             .await?;
//         guilds.append(&mut batch);
//         last_guild_id = batch.last().map(|guild| *guild.id.as_u64());
//     }

