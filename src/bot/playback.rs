// use std::{collections::HashMap, convert::TryInto, env, sync::Arc};

// use serenity::{
//     async_trait,
//     client::{Client, Context, EventHandler},
//     framework::{
//         StandardFramework,
//         standard::{
//             Args, CommandResult,
//             macros::{command, group},
//         },
//     },
//     model::{channel::Message, gateway::Ready, misc::Mentionable},
//     prelude::Mutex,
//     Result as SerenityResult,
// };

// use songbird::{
//     input::{
//         self,
//         cached::{Compressed, Memory},
//         Input,
//     },
//     Bitrate,
//     Call,
//     Event,
//     EventContext,
//     EventHandler as VoiceEventHandler,
//     SerenityInit,
//     TrackEvent,
// };

// struct SoundStore;

// enum CachedSound {
//     Compressed(Compressed),
//     Uncompressed(Memory),
// }

// impl TypeMapKey for SoundStore {
//     type Value = Arc<Mutex<HashMap<String, CachedSound>>>;
// }

// async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
// }
