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

#[derive(Debug)]
pub enum CachedSound {
    Compressed(Compressed),
    Uncompressed(Memory),
}

#[derive(Debug)]
pub struct CachedTrack {
    pub source: CachedSound,
    pub duration: u128,
    pub handle: Option<TrackHandle>,
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

impl CachedTrack {
    pub fn new(source: CachedSound, duration: u128) -> Self {
        CachedTrack {
            source,
            duration,
            handle: None,
        }
    }

    /// Plays a track by source in an active call and
    /// updates the `handle` field with the recieved `TrackHandle`.
    ///
    /// # Safety note
    /// The source field must not have been changed or deleted after creation
    /// and `call` must be an active mutable reference to a on-going discord voice call.
    ///
    /// # Example
    /// ```rust
    /// let cached_track = CachedTrack::new(CachedSound::new(memory), duration);
    /// cached_track.play(&mut call); // converts the source, plays the track and stores the handle
    /// ```
    pub fn play(&mut self, call: &mut Call) {
        self.handle = Some(call.play_source((&self.source).into()));
    }

    /// Stops the playback of the track if it has a valid track handle.
    ///
    /// # Safety note
    /// The source field must not have been changed or deleted after creation
    /// and `call` must be an active mutable reference to a on-going discord voice call.
    ///
    /// # Example
    /// ```rust
    /// let cached_track = CachedTrack::new(CachedSound::new(memory), duration);
    /// cached_track.play(&mut call);
    /// ...
    /// // lookup the cached track from e.g a HashMap
    /// cached_track.stop();
    /// ```
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

/// Plays a song by path in the currently active discord voice call (if any).
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

/// Stops the playback of a track by path in the current discord voice call.
pub async fn stop(cached_tracks: &mut HashMap<String, CachedTrack>, path: &String) -> Result<(), String> {
    match cached_tracks.get_mut(path) {
        None => Err(format!("The track is not playing: {}", path)),
        Some(cached_track) => cached_track.stop().await,
    }
}

/// Stops the playback of all currently playing tracks.
pub async fn stop_all(cached_tracks: &mut HashMap<String, CachedTrack>) {
    for (_, cached_track) in cached_tracks.iter_mut() {
        let _ = cached_track.stop().await;
    }
}
