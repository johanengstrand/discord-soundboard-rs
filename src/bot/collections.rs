use crate::bot::tracks::Track;
use config::{Config, File, ConfigError};

use std::{
    fs,
    path::PathBuf,
    collections::HashMap,
};

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

fn read_collection(tracks: &mut Vec<Track>, path: &PathBuf) {
    if let Ok(collection) = Collection::new(path) {
        // TODO: Go through each track path in the collection and check if it is in tracks.
        //       If so, call track.add_collection(<name of collection>);
    }
}

pub fn add_collections_to_tracks(tracks: &mut Vec<Track>) -> HashMap<PathBuf, Vec<PathBuf>> {
    // TODO: Find and loop through all collections with read_collection
    // if dir.is_dir() {
    //     for entry in fs::read_dir(dir)? {
    //         let entry = entry?;
    //         let path = entry.path();
    //         let name = entry.file_name().into_string()
    //             .expect("Could not convert file name to string");
    //         if name.get(0..1) != Some(".") {
    //             if path.is_dir() {
    //                 visit_dirs(&path, &category.join(name), tracks)?;
    //             } else {
    //                 // TODO: Add 'favorites' category if track is a favorite
    //                 let keyword = category.to_path_buf();
    //                 let categories = vec!(keyword);
    //                 tracks.push(Track { name, categories, path });
    //             }
    //         }
    //     }
    //     Ok(())
    // } else {
    //     Err(Error::new(ErrorKind::NotFound, "No such path"))
    // }
    HashMap::new()
}
