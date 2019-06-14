mod fs;
mod mem;
mod list;


use std::io;
use std::path::{Path, PathBuf};

pub trait Hook {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)>;
}

pub use fs::FsHook;
pub use mem::MemHook;
pub use list::ListHook;
