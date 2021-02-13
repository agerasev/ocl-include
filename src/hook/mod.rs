mod fs;
mod list;
mod mem;

use std::io;
use std::path::{Path, PathBuf};

/// Something that may provide file content by its name
pub trait Hook {
    /// Performs file loading
    ///
    /// Arguments:
    /// + `path`: absolute or relative file path,
    /// + `dir`: directory of parent file if it contains relative include directive
    ///
    /// Returns on success:
    /// + Absolute path to file
    /// + File content
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)>;
}

pub use fs::{FsHook, FsHookBuilder};
pub use list::{ListHook, ListHookBuilder};
pub use mem::{MemHook, MemHookBuilder};
