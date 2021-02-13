use std::collections::hash_map::{Entry, HashMap};
use std::io;
use std::path::{Path, PathBuf};

use super::Hook;

/// Hook for retrieving files from memory
pub struct MemHook {
    files: HashMap<PathBuf, String>,
}

impl Default for MemHook {
    fn default() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}

impl MemHook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> MemHookBuilder {
        MemHookBuilder { hook: Self::new() }
    }

    pub fn add_file(&mut self, name: &Path, data: String) -> io::Result<()> {
        match self.files.entry(name.to_path_buf()) {
            Entry::Occupied(_) => Err(io::ErrorKind::AlreadyExists.into()),
            Entry::Vacant(v) => {
                v.insert(data);
                Ok(())
            },
        }
    }

    fn read_file(&self, path: &Path) -> Option<String> {
        self.files.get(path).cloned()
    }
}

pub struct MemHookBuilder {
    hook: MemHook,
}

impl MemHookBuilder {
    pub fn add_file(mut self, name: &Path, data: String) -> io::Result<Self> {
        self.hook.add_file(name, data).map(|()| self)
    }
    pub fn build(self) -> MemHook {
        self.hook
    }
}

impl Hook for MemHook {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        dir.and_then(|dir| {
            let path = dir.join(path);
            self.read_file(&path).map(|data| (path, data))
        })
        .or_else(|| {
            self.files
                .get(path)
                .map(|data| (path.to_path_buf(), data.clone()))
        })
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("path: {:?}, dir: {:?}", path, dir),
            )
        })
    }
}
