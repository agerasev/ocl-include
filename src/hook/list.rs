use std::{
    io,
    path::{Path, PathBuf},
};

use super::Hook;

/// Hook for retrieving files from list of other hooks subsequently
pub struct ListHook {
    hooks: Vec<Box<dyn Hook>>,
}

impl Default for ListHook {
    fn default() -> Self {
        Self { hooks: Vec::new() }
    }
}

impl ListHook {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn builder() -> ListHookBuilder {
        ListHookBuilder { hook: Self::new() }
    }
    pub fn add_hook<T: 'static + Hook>(&mut self, hook: T) {
        self.hooks.push(Box::new(hook));
    }
}

pub struct ListHookBuilder {
    hook: ListHook,
}

impl ListHookBuilder {
    pub fn add_hook<T: 'static + Hook>(mut self, hook: T) -> Self {
        self.hook.add_hook(hook);
        self
    }
    pub fn build(self) -> ListHook {
        self.hook
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
                },
            }
        }

        match res {
            Some(x) => x,
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("path: {:?}, dir: {:?}", path, dir),
            )),
        }
    }
}
