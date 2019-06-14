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

    pub fn add_file(mut self, name: &Path, data: String) -> io::Result<Self> {
        match self.files.insert(name.to_path_buf(), data) {
            Some(_) => Err(io::ErrorKind::AlreadyExists.into()),
            None => Ok(self),
        }
    }

    fn read_file(&self, path: &Path) -> Option<String> {
        self.files.get(path).map(|data| data.clone())
    }
}

impl Hook for MemHook {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        dir
        .and_then(|dir| {
            let path = dir.join(path);
            self.read_file(&path)
            .map(|data| (path, data))
        })
        .or_else(|| {
            self.files.get(path)
            .map(|data| (path.to_path_buf(), data.clone()))
        })
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            format!("path: {:?}, dir: {:?}", path, dir),
        ))
    }
}
