use std::collections::HashMap;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};

use super::{Hook};


/// Hook to search for included files in file system.
pub struct FsHook {
    dirs: Vec<PathBuf>,
    cache: HashMap<PathBuf, Option<String>>,
}

pub struct FsHandle {
    path: PathBuf,
}

impl FsHook {
    pub fn new() -> Self {
        FsHook { dirs: Vec::new() }
    }

    pub fn add_dir(&mut self, dir: &Path) -> io::Result<()> {
        Self::check_dir(dir)?;
        self.dirs.push(dir.to_path_buf());
        Ok(())
    }

    fn check_dir(dir: &Path) -> io::Result<()> {
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

    fn check_file(path: &Path) -> io::Result<()> {
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

    fn find_in_dir(self, dir: &Path, name: &Path) -> io::Result<Option<PathBuf>> {
        let path = dir.join(name);
        match Self::check_file(&path) {
            Ok(()) => Ok(Some(path)),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Ok(None),
                _ => return Err(e),
            }
        }
    }

    fn find_file(&self, dir: Option<&Path>, name: &Path) -> io::Result<PathBuf> {
        match dir {
            Some(dir) => match Self::find_in_dir(dir, name)? {
                Some(path) => return Ok(path),
                None => (),
            },
            None => (),
        }

        for dir in self.dirs.iter() {
            match Self::find_in_dir(dir, name)? {
                Some(path) => return Ok(path),
                None => (),
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "File '{}' not found in any of include dirs",
                name.display()
            ),
        ))
    }
}

impl Hook for FsHook {
    type Handle = FsHandle;
    fn find(&self, dir: Option<&Path>, name: &Path) -> io::Result<Self::Handle> {
        self.find_file(dir, name).map(|path| FsHandle { path })
    }
}

impl Handle for FsHandle {
    fn path(&self) -> &Path {
        self.path.as_path()
    }
    fn read(self) -> io::Result<String> {
        fs::read_to_string(self.path)
    }
}
