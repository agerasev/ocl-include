use std::{
    io,
    collections::hash_map::{HashMap, Entry},
    path::{Path, PathBuf},
};


pub struct CacheEntry {
    pub text: String,
    pub occured: usize,
}

impl CacheEntry {
    fn new(text: String) -> Self {
        Self { text, occured: 1 }
    }
}

pub struct Cache {
    map: HashMap<PathBuf, CacheEntry>,
}

impl Cache {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
    pub fn put(&mut self, path: &Path, text: String) -> io::Result<()> {
        match self.map.entry(path.to_path_buf()) {
            Entry::Occupied(_) => Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                path.to_string_lossy(),
            )),
            Entry::Vacant(v) => {
                v.insert(CacheEntry::new(text));
                Ok(())
            },
        }
    }
    pub fn get(&self, path: &Path) -> Option<&CacheEntry> {
        self.map.get(path)
    }
    pub fn get_mut(&mut self, path: &Path) -> Option<&mut CacheEntry> {
        self.map.get_mut(path)
    }
}
