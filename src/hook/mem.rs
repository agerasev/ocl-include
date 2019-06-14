use std::io;
use std::path::{Path, PathBuf};
use std::collections::{HashMap};

use super::{Hook};


pub struct MemHook {
    files: HashMap<PathBuf, String>,
}

impl MemHook {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn add_file(&mut self, name: &Path, data: String) -> io::Result<()> {
        match self.files.insert(name.to_path_buf(), data) {
            Some(_) => Err(io::ErrorKind::AlreadyExists.into()),
            None => Ok(()),
        }
    }
}

impl Hook for MemHook {
    fn read(&self, path: &Path, _dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        match self.files.get(path) {
            Some(data) => Ok((path.to_path_buf(), data.clone())),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                path.to_string_lossy(),
            )),
        }
    }
}
