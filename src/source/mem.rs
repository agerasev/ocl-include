use std::{
    collections::hash_map::{Entry, HashMap},
    io,
    path::{Path, PathBuf},
};

use super::Source;

/// Source for retrieving files from memory.
#[derive(Default)]
pub struct Mem {
    files: HashMap<PathBuf, String>,
}

impl Mem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> MemBuilder {
        MemBuilder {
            source: Self::new(),
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, name: &P, data: String) -> io::Result<()> {
        match self.files.entry(name.as_ref().to_path_buf()) {
            Entry::Occupied(_) => Err(io::ErrorKind::AlreadyExists.into()),
            Entry::Vacant(v) => {
                v.insert(data);
                Ok(())
            }
        }
    }

    fn read_file<P: AsRef<Path>>(&self, path: &P) -> Option<String> {
        self.files.get(path.as_ref()).cloned()
    }
}

pub struct MemBuilder {
    source: Mem,
}

impl MemBuilder {
    pub fn add_file<P: AsRef<Path>>(mut self, name: &P, data: String) -> io::Result<Self> {
        self.source.add_file(name, data).map(|()| self)
    }

    pub fn build(self) -> Mem {
        self.source
    }
}

impl Source for Mem {
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
