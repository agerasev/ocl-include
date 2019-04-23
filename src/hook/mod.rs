mod mem;
mod fs;

use std::io;
use std::path::{Path, PathBuf};

pub trait Hook {
    fn get(&self, dir: Option<&Path>, name: &Path) -> io::Result<Option<(PathBuf, String)>>;
}
