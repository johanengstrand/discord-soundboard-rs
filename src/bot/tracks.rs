use std::fs;
use std::path::PathBuf;
use std::io::{self, Error, ErrorKind};

fn visit_dirs(dir: &PathBuf, tracks: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, tracks)?;
            } else {
                tracks.push(path);
            }
        }
        Ok(())
    } else {
        Err(Error::new(ErrorKind::NotFound, "No such path"))
    }
}

pub fn get_tracks_in_dir(dir: &str) -> Result<Vec<PathBuf>, String> {
    let mut tracks = Vec::new();
    match visit_dirs(&PathBuf::from(dir), &mut tracks) {
        Ok(_) => Ok(tracks),
        Err(why) => Err(format!("Failed to read tracks in {}: {}", dir, why)),
    }
}
