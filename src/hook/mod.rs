use std::io;
use std::path::{Path, PathBuf};

pub trait Hook {
	fn find(&self, path: &Path, dir: Option<&Path>) -> io::Result<PathBuf>;
	fn read(&self, full_path: &Path) -> io::Result<String>;
}
