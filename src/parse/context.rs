use super::{
    file_context::FileContext,
    parser::{FileCache, FileCacheEntry, Flags},
};
use crate::{node::Node, source::Source};
use std::{
    collections::hash_map::Entry,
    io,
    path::{Path, PathBuf},
};

pub struct Context<'a> {
    source: &'a dyn Source,
    file_cache: &'a mut FileCache,
    file_stack: Vec<PathBuf>,
    flags: &'a Flags,
}

impl<'a> Context<'a> {
    pub fn new(source: &'a dyn Source, flags: &'a Flags, file_cache: &'a mut FileCache) -> Self {
        Self {
            source,
            file_cache,
            file_stack: Vec::new(),
            flags,
        }
    }

    pub fn flags(&self) -> &'a Flags {
        self.flags
    }
    pub fn is_file_occured(&self, path: &Path) -> bool {
        self.file_cache.get(path).unwrap().occured > 1
    }

    fn read_file(&mut self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        self.source.read(path, dir).map(|(path, text)| {
            match self.file_cache.entry(path.clone()) {
                Entry::Occupied(mut v) => {
                    v.get_mut().occured += 1;
                }
                Entry::Vacant(v) => {
                    v.insert(FileCacheEntry::new());
                }
            }
            (path, text)
        })
    }

    fn parse_file(&mut self, path: &Path, text: String) -> io::Result<Option<Node>> {
        FileContext::new(path, self).parse(text)
    }

    pub fn build_tree(&mut self, path: &Path, dir: Option<&Path>) -> io::Result<Option<Node>> {
        self.read_file(path, dir)
            .and_then(|(path, text)| {
                if self.file_stack.iter().filter(|p| **p == path).count() >= 2 {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "recursion found",
                    ))
                } else {
                    self.file_stack.push(path.clone());
                    Ok((path, text))
                }
            })
            .and_then(|(path, text)| self.parse_file(&path, text).map(|x| (x, path)))
            .map(|(x, path)| {
                assert_eq!(self.file_stack.pop().unwrap(), path);
                x
            })
    }
}
