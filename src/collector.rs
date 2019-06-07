use std::{
    io,
    env,
    ops::Range,
    collections::hash_map::{HashMap, Entry},
    path::{Path, PathBuf},
};

use crate::hook::{Hook};

pub struct Node {
    name: PathBuf,
    range: Range<usize>,
    nodes: Vec<Node>,
}

pub struct Data {
    lines: Vec<String>,
}

impl Data {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

struct Cache {
    map: HashMap<PathBuf, String>,
}

impl Cache {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
    pub fn put(&mut self, path: &Path, text: String) -> io::Result<()> {
        match self.map.entry(path) {
            Entry::Occupied(_) => Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                path.to_string_lossy(),
            )),
            Entry::Vacant(v) => {
                v.insert(text);
                Ok(())
            },
        }
    }
    pub fn get(&mut self, path: &Path) -> Option<&String> {
        self.map.get(path)
    }
}

pub struct Collector {
    hook: Box<dyn Hook>,
    cache: Cache,
}

impl Collector {
    pub fn new(hook: Box<dyn Hook>) -> Self {
        Collector { hook, cache: Cache::new() }
    }
    pub fn collect(&mut self, data: &mut Data, path: &Path, dir: Option<&Path>) -> io::Result<Node> {
        if path.is_absolute() {
            Ok(path)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput,format!(
                "absolute path expected, got '{}'", path.to_string_lossy()
            )))
        }
        .and_then(|path| self.hook.find(path, dir))
        .and_then(|full_path| self.parse(data, &full_path))
    }
    pub fn parse(&mut self, data: &mut Data, path: &Path) -> io::Result<Node> {
        self.hook.read()
        .and_then(|text| self.cache
    }
}

pub fn collect(main: &Path, hook: Box<dyn Hook>) -> io::Result<(Node, Data)> {
    if main.is_absolute() {
        Ok(main.to_path_buf())
    } else {
        env::current_dir()
        .map(|cwd| cwd.join(main))
    }
    .and_then(|p| p.canonicalize())
    .and_then(|p| {
        let data = Data::new();
        Collector::new(hook).collect(&mut data, &p, None)
        .map(|tree| (tree, data))
    })
}
