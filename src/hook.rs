use std::io;
use std::path::{Path, PathBuf};

pub trait Hook {
	fn find(dir: Option<&Path>, rel_path: &Path) -> Option<PathBuf>;
	fn load(full_path: &Path) -> io::Result<String>;
}
