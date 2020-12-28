use std::fs;
use std::path::PathBuf;
use std::io::{self, Error, ErrorKind};

#[derive(Debug, Serialize)]
pub struct Track {
    name: String,
    category: PathBuf,
    path: PathBuf,
}

fn visit_dirs(dir: &PathBuf, category: &PathBuf, tracks: &mut Vec<Track>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().into_string().unwrap();
            if path.is_dir() {
                visit_dirs(&path, &category.join(file_name), tracks)?;
            } else {
                tracks.push(Track {
                    name: file_name,
                    category: category.to_path_buf(),
                    path: path,
                });
            }
        }
        Ok(())
    } else {
        Err(Error::new(ErrorKind::NotFound, "No such path"))
    }
}

pub fn get_tracks_in_dir(dir: &str) -> Result<Vec<Track>, String> {
    let mut tracks = Vec::new();
    match visit_dirs(&PathBuf::from(dir), &PathBuf::new(), &mut tracks) {
        Ok(_) => Ok(tracks),
        Err(why) => Err(format!("Failed to read tracks in {}: {}", dir, why)),
    }
}
