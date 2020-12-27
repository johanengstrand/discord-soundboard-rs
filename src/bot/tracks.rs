use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct Track {
    name: String,
    path: PathBuf,
}

pub fn get_tracks_in_dir(dir: &str) -> Result<Vec<Track>, String> {
    let path = PathBuf::from(dir);
    let mut tracks = Vec::new();

    match fs::read_dir(path) {
        Err(why) => Err(format!("Failed to read tracks in {}: {}", dir, why)),
        Ok(paths) => {
            for path in paths {
                let track_info = path.unwrap();
                tracks.push(Track {
                    name: track_info.file_name().into_string().unwrap(),
                    path: track_info.path(),
                });
            };

            Ok(tracks)
        },
    }
}
