use std::fs::File;
use std::io;
use std::path::Path;

static ERROR_MESSAGE: &str = "Unknown Architecture";

#[derive(Debug)]
pub struct Handle;

impl Eq for Handle {}

impl PartialEq for Handle {
    fn eq(&self, _other: &Handle) -> bool {
        false
    }
}

impl Handle {
    pub fn from_path<P: AsRef<Path>>(_p: P) -> io::Result<Handle> {
        error()
    }

    pub fn from_file(_file: File) -> io::Result<Handle> {
        error()
    }

    pub fn stdin() -> io::Result<Handle> {
        error()
    }

    pub fn stdout() -> io::Result<Handle> {
        error()
    }

    pub fn stderr() -> io::Result<Handle> {
        error()
    }

    pub fn as_file(&self) -> &File {
        panic!(ERROR_MESSAGE);
    }

    pub fn as_file_mut(&self) -> &mut File {
        panic!(ERROR_MESSAGE);
    }
}

fn error<T>() -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::Other, ERROR_MESSAGE))
}