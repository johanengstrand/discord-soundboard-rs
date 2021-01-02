use crate::bot;

use std::{
    collections::HashMap,
    convert::TryInto,
    sync::Arc,
};

use serenity::{
    client::Client,
    prelude::{Mutex, TypeMapKey},
};

use songbird::{
    tracks::TrackHandle,
    input::{
        self,
        cached::{Compressed, Memory},
        Input,
    },
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

pub async fn play(bot_state_lock: &mut bot::State, path: &String)
    -> Result<u128, String> {
    match &bot_state_lock.current_call {
        Some(call_lock) => {
            let mut call = call_lock.lock().await;
            let source = input::ffmpeg(path).await.expect("File should be in root folder.");
            let duration = source.metadata.duration;
            let ting_src = Memory::new(source).expect("These parameters are well-defined.");
            let _ = ting_src.raw.spawn_loader();
            let cached_track = CachedSound::Uncompressed(ting_src);
            let song = call.play_source((&cached_track).into());
            bot_state_lock.current_tracks.insert(path.to_string(), song);

            match duration {
                None => Ok(0),
                Some(duration) => Ok(duration.as_millis()),
            }
        },
        None => Err(String::from("The bot is not currently connected to a voice channel")),
    }

}

pub async fn stop(current_tracks: &mut HashMap<String, TrackHandle>, path: &String) -> Result<(), String> {
    let track = current_tracks.get(path);

    match track {
        Some(track) => {
            let track_state = track.get_info().await;
            match track_state {
                Ok(state) => {
                    if !state.playing.is_done() {
                        // TODO: Add error handling
                        return match track.stop() {
                            Ok(_) => Ok(()),
                            Err(why) => Err(format!("Could not stop track: {}", why)),
                        };
                    }

                    current_tracks.remove(path);
                    Ok(())
                },
                Err(why) => Err(format!("Could not fetch track state: {}", why)),
            }
        },
        None => Err(format!("The track is not playing: {}", path)),
    }
}
