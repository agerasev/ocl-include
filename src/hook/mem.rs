use std::io;
use std::path::{Path, PathBuf};
use std::collections::{HashMap};

use super::{Hook};

pub struct MemHook {
    files: HashMap<PathBuf, String>,
}

pub struct MemHandle {
    path: PathBuf,
    data: String,
}

impl MemHook {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn add(&mut self, path: &Path, data: String) {
        self.files.insert(path.to_path_buf(), data);
    }
}

impl Hook for MemHook {
    fn read(&self, dir: Option<&Path>, name: &Path) -> io::Result<(PathBuf, String)> {
        match self.files.get(name) {
            Some(data) => Ok((name.to_path_buf(), data.clone())),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File '{}' not found", name.display()),
            )),
        }
    }
}
