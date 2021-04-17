use super::Source;
use std::{io, rc::Rc};
use uni_path::{Path, PathBuf};

/// Vector of sources is also source.
impl<S: Source> Source for Vec<S> {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        let mut res = None;
        for source in self.iter() {
            match source.read(path, dir) {
                Ok(x) => {
                    res = Some(Ok(x));
                    break;
                }
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
                format!("path: {}, dir: {:?}", path, dir),
            )),
        }
    }
}

#[macro_export]
macro_rules! source_vec {
    ($($x:expr),* $(,)?) => {
        vec![ $(Box::new($x) as Box::<dyn $crate::Source>),* ]
    };
}

/// Source reference is also source.
impl<'a, S: Source> Source for &'a S {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        (*self).read(path, dir)
    }
}

/// Source Box is also source.
impl Source for Box<dyn Source> {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        self.as_ref().read(path, dir)
    }
}

/// Source Rc is also source.
impl<S: Source> Source for Rc<S> {
    fn read(&self, path: &Path, dir: Option<&Path>) -> io::Result<(PathBuf, String)> {
        self.as_ref().read(path, dir)
    }
}
