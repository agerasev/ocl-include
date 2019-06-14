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

impl FsHook {
    pub fn new() -> Self {
        FsHook {
            inc_dirs: Vec::new(),
            cache: Cell::new(Some(HashMap::new())),
        }
    }

    pub fn include_dir(mut self, dir: &Path) -> io::Result<Self> {
        self.check_dir(dir)?;
        self.inc_dirs.push(dir.to_path_buf());
        Ok(self)
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
                _ => return Err(e),
            }
        }
    }

    fn find_file(&self, dir: Option<&Path>, name: &Path) -> io::Result<PathBuf> {
        if name.is_absolute() {
            return Ok(name.to_path_buf())
        }

        match dir {
            Some(dir) => match self.find_in_dir(dir, name)? {
                Some(path) => return Ok(path),
                None => (),
            },
            None => (),
        }

        for dir in self.inc_dirs.iter() {
            match self.find_in_dir(dir, name)? {
                Some(path) => return Ok(path),
                None => (),
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, name.to_string_lossy()))
    }
}

impl Hook for FsHook {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        self.find_file(dir, path)
        .and_then(|path| {
            let mut map = self.cache.take().unwrap();
            
            let res = match map.entry(path.to_path_buf()) {
                Entry::Occupied(v) => Ok(v.get().clone()),
                Entry::Vacant(v) => {
                    fs::read_to_string(&path)
                    .and_then(|data| {
                        v.insert(data.clone());
                        Ok(data)
                    })
                }
            }
            .map(|data| (path, data));

            self.cache.set(Some(map));
            res
        })
    }
}
