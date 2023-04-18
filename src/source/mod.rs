pub mod container;
pub mod fs;
pub mod mem;

use std::{
    io,
    path::{Path, PathBuf},
};

/// Something that may provide file content by its name.
pub trait Source {
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

pub use fs::Fs;
pub use mem::Mem;
