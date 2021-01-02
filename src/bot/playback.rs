use crate::bot;

use std::{
    collections::HashMap,
    convert::TryInto,
};

use songbird::{
    Call,
    tracks::TrackHandle,
    input::{
        self,
        cached::{Compressed, Memory},
        Input,
    },
};

pub enum CachedSound {
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

pub struct CachedTrack {
    pub source: CachedSound,
    pub duration: u128,
    pub handle: Option<TrackHandle>,
}

impl CachedTrack {
    pub fn new(source: CachedSound, duration: u128) -> Self {
        CachedTrack {
            source,
            duration,
            handle: None,
        }
    }

    pub fn play(&mut self, call: &mut Call) {
        self.handle = Some(call.play_source((&self.source).into()));
    }

    pub async fn stop(&mut self) -> Result<(), String> {
        match &self.handle {
            Some(track_handle) => {
                match track_handle.get_info().await {
                    Ok(state) => {
                        if !state.playing.is_done() {
                            return match track_handle.stop() {
                                Ok(_) => {
                                    self.handle = None;
                                    Ok(())
                                },
                                Err(why) => Err(format!("Could not stop track: {}", why)),
                            };
                        }
                        self.handle = None;
                        Ok(())
                    },
                    Err(why) => Err(format!("Could not fetch track state: {}", why)),
                }
            },
            None => Err(String::from("The track is currently not playing")),
        }
    }
}

pub async fn play(bot_state_lock: &mut bot::State, path: &String)
    -> Result<u128, String> {
    match &bot_state_lock.current_call {
        Some(call_lock) => {
            let mut call = call_lock.lock().await;
            match bot_state_lock.cached_tracks.get_mut(path) {
                None => {
                    // TODO: Compressed?
                    let source = input::ffmpeg(path).await.expect("File should be in root folder.");
                    let duration = match source.metadata.duration {
                        None => 0,
                        Some(duration) => duration.as_millis(),
                    };

                    let track_source = Memory::new(source).expect("These parameters are well-defined.");
                    let _ = track_source.raw.spawn_loader();
                    let mut cached_track = CachedTrack::new(CachedSound::Uncompressed(track_source), duration);

                    cached_track.play(&mut call);
                    bot_state_lock.cached_tracks.insert(path.clone(), cached_track);

                    Ok(duration)
                },
                Some(cached_track) => {
                    cached_track.play(&mut call);
                    Ok(cached_track.duration)
                },
            }
        },
        None => Err(String::from("The bot is not currently connected to a voice channel")),
    }
}

pub async fn stop(cached_tracks: &mut HashMap<String, CachedTrack>, path: &String) -> Result<(), String> {
    match cached_tracks.get_mut(path) {
        None => Err(format!("The track is not playing: {}", path)),
        Some(cached_track) => cached_track.stop().await,
    }
}
