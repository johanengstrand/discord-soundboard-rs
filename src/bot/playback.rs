use std::{
    collections::HashMap,
    convert::TryInto,
    sync::Arc
};

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{channel::Message, gateway::Ready, misc::Mentionable},
    prelude::{Mutex, TypeMapKey, SerenityError},
    Result as SerenityResult,
};

use songbird::{
    input::{
        self,
        cached::{Compressed, Memory},
        Input,
    },
    Songbird,
    Bitrate,
    Call,
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
    SerenityInit,
    TrackEvent,
};

enum CachedSound {
    Compressed(Compressed),
    Uncompressed(Memory),
}

impl From<&CachedSound> for Input {
    fn from(obj: &CachedSound) -> Self {
        use CachedSound::*;
        match obj {
            Compressed(c) => c.new_handle()
                .into(),
            Uncompressed(u) => u.new_handle()
                .try_into()
                .expect("Failed to create decoder for Memory source."),
        }
    }
}

struct SoundStore;
impl TypeMapKey for SoundStore {
    type Value = Arc<Mutex<HashMap<String, CachedSound>>>;
}

pub async fn setup(client: &mut Client) {
    let mut data = client.data.write().await;
    let audio_map = HashMap::new();
    data.insert::<SoundStore>(Arc::new(Mutex::new(audio_map)));
}

pub async fn play(call_lock: Arc<Mutex<Call>>, path: &String) -> Result<(), SerenityError> {
    let mut call = call_lock.lock().await;
    let ting_src = Memory::new(
        input::ffmpeg(path).await.expect("File should be in root folder."),
    ).expect("These parameters are well-defined.");
    let _ = ting_src.raw.spawn_loader();
    let cached_track = CachedSound::Uncompressed(ting_src);
    // audio_map.insert("ting".into(), CachedSound::Uncompressed(ting_src));
    let song = call.play_source((&cached_track).into());
    Ok(())
}

