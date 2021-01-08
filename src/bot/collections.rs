use crate::bot::tracks::Track;
use std::{fs, path::PathBuf};
use config::{Config, File, ConfigError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Collection {
    tracks: Vec<PathBuf>,
}

impl Collection {
    pub fn new(path: &PathBuf) -> Result<Self, ConfigError> {
        let mut collection = Config::default();
        match path.to_str() {
            Some(collection_string) => {
                match collection.merge(File::with_name(collection_string)) {
                    Err(why) => {
                        println!("Failed to parse collection: {}", why);
                        Err(why)
                    },
                    Ok(_) => collection.try_into(),
                }
            },
            None => Err(ConfigError::Message(String::from("Could not convert collection name to String"))),
        }
    }
}

// TODO: Add error handling
fn add_collection_to_track(tracks: &mut Vec<Track>, collection_name: &PathBuf) {
    // TODO: Error handling
    if let Ok(collection) = Collection::new(collection_name) {
        for path in collection.tracks.iter() {
            for track in tracks.iter_mut() {
                if track.path == *path {
                    track.add_collection(collection_name.to_path_buf());
                    break;
                }
            }
        }
    }
}

pub fn add_collections_to_tracks(tracks: &mut Vec<Track>) -> Result<(), std::io::Error> {
    // TODO: Find and loop through all collections with read_collection
    for collection in fs::read_dir(PathBuf::from("config/collections"))? {
        let collection = collection?;
        // TODO: Add error handling
        add_collection_to_track(tracks, &collection.path());
    }

    Ok(())
}
