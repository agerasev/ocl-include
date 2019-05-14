use std::io;
use std::path::{Path, PathBuf};
use std::collections::{HashMap};

use super::{Hook};

pub struct MemHook {
    prefix: PathBuf,
    files: HashMap<PathBuf, String>,
}

impl MemHook {
    pub fn new(prefix: &Path) -> Self {
        Self {
            prefix: prefix.to_path_buf(),
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, name: &Path, data: String) {
        self.files.insert(name.to_path_buf(), data);
    }
}

impl Hook for MemHook {
    fn get(&self, _dir: Option<&Path>, path: &Path) -> io::Result<Option<(PathBuf, String)>> {
        Ok(path.strip_prefix(&self.prefix).ok()
        .and_then(|name| {
            self.files.get(name).map(|data| {
                (name.to_path_buf(), data.clone())
            })
        }))
    }
}
