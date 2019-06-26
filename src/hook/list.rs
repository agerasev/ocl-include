use std::{
    io,
    path::{Path, PathBuf},
};

use super::{Hook};

/// Hook for retrieving files from list of other hooks subsequently
pub struct ListHook {
    hooks: Vec<Box<dyn Hook>>,
}

impl ListHook {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }
    pub fn add_hook<T: 'static + Hook>(mut self, hook: T) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }
}

impl Hook for ListHook {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        let mut res = None;
        for hook in self.hooks.iter() {
            match hook.read(path, dir) {
                Ok(x) => {
                    res = Some(Ok(x));
                    break;
                },
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => continue,
                    _ => {
                        res = Some(Err(e));
                        break;
                    }
                }
            }
        }

        res.unwrap_or(Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("path: {:?}, dir: {:?}", path, dir),
        )))
    }
}
