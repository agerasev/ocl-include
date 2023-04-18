use super::context::Context;
use crate::{node::Node, source::Source};
use std::{
    cell::RefCell,
    collections::hash_map::HashMap,
    io,
    path::{Path, PathBuf},
};

pub struct FileCacheEntry {
    pub occured: usize,
}
impl FileCacheEntry {
    pub fn new() -> Self {
        Self { occured: 1 }
    }
}
pub type FileCache = HashMap<PathBuf, FileCacheEntry>;

pub type Flags = HashMap<String, bool>;

pub struct Parser {
    source: Box<dyn Source>,
    flags: Flags,
    file_cache: RefCell<FileCache>,
}

#[derive(Default)]
pub struct ParserBuilder {
    sources: Vec<Box<dyn Source>>,
    flags: Flags,
}

impl ParserBuilder {
    pub fn add_source<S: Source + 'static>(mut self, source: S) -> Self {
        self.sources.push(Box::new(source));
        self
    }

    pub fn add_flag(mut self, name: String, value: bool) -> Self {
        self.flags.insert(name, value);
        self
    }

    pub fn build(self) -> Parser {
        Parser::new(Box::new(self.sources), self.flags)
    }
}

impl Parser {
    pub fn new(source: Box<dyn Source>, flags: Flags) -> Self {
        Self {
            source,
            flags,
            file_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn builder() -> ParserBuilder {
        ParserBuilder::default()
    }

    /// Reads and parses source files and resolves dependencies.
    ///
    /// Returns node tree that could be collected into resulting code string and index.
    pub fn parse(&self, main: &Path) -> io::Result<Node> {
        let mut file_cache = self.file_cache.borrow_mut();
        let mut context = Context::new(self.source.as_ref(), &self.flags, &mut file_cache);
        context.build_tree(main, None).and_then(|root| {
            root.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Root file {:?} not found", main),
                )
            })
        })
    }
}
