use std::{
    io,
    fs,
    cell::Cell,
    path::{Path, PathBuf},
    collections::hash_map::{HashMap, Entry},
};

use super::{Hook};


/// Hook for reading files from filesystem
pub struct FsHook {
    inc_dirs: Vec<PathBuf>,
    cache: Cell<Option<HashMap<PathBuf, String>>>,
}

impl Default for FsHook {
    fn default() -> Self {
        FsHook {
            inc_dirs: Vec::new(),
            cache: Cell::new(Some(HashMap::new())),
        }
    }
}

impl FsHook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> FsHookBuilder {
        FsHookBuilder { hook: Self::new() }
    }

    pub fn include_dir(&mut self, dir: &Path) -> io::Result<()> {
        self.check_dir(dir)?;
        self.inc_dirs.push(dir.to_path_buf());
        Ok(())
    }

    fn check_dir(&self, dir: &Path) -> io::Result<()> {
        let meta = fs::metadata(dir)?;
        if !meta.is_dir() {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("'{}' is not a directory", dir.display()),
            ))
        } else {
            Ok(())
        }
    }

    fn check_file(&self, path: &Path) -> io::Result<()> {
        let map = self.cache.take().unwrap();
        let contains = map.contains_key(path);
        self.cache.set(Some(map));
        if contains {
            return Ok(());
        }

        match fs::metadata(path) {
            Ok(meta) => {
                if !meta.is_file() {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("'{}' is not a file", path.display()),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    }

    fn find_in_dir(&self, dir: &Path, name: &Path) -> io::Result<Option<PathBuf>> {
        let path = dir.join(name);
        match self.check_file(&path) {
            Ok(()) => Ok(Some(path)),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Ok(None),
                _ => Err(e),
            }
        }
    }

    fn find_file(&self, dir: Option<&Path>, name: &Path) -> io::Result<PathBuf> {
        if name.is_absolute() {
            return Ok(name.to_path_buf())
        }

        if let Some(dir) = dir {
            if let Some(path) = self.find_in_dir(dir, name)? {
                return Ok(path);
            }
        }

        for dir in self.inc_dirs.iter() {
            if let Some(path) = self.find_in_dir(dir, name)? {
                return Ok(path);
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, name.to_string_lossy()))
    }
}

pub struct FsHookBuilder {
    hook: FsHook,
}

impl FsHookBuilder {
    pub fn include_dir(mut self, dir: &Path) -> io::Result<Self> {
        self.hook.include_dir(dir).map(|()| self)
    }

    pub fn build(self) -> FsHook {
        self.hook
    }
}

impl Hook for FsHook {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        self.find_file(dir, path)
        .and_then(|path| {
            let mut map = self.cache.take().unwrap();
            
            let res = match map.entry(path.clone()) {
                Entry::Occupied(v) => Ok(v.get().clone()),
                Entry::Vacant(v) => {
                    fs::read_to_string(&path)
                    .map(|data| {
                        v.insert(data.clone());
                        data
                    })
                }
            }
            .map(|data| (path, data));

            self.cache.set(Some(map));
            res
        })
    }
}
