use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::path::Path;

use libc;

#[derive(Debug)]
pub struct Handle {
    file: File,
    dev: u64,
    ino: u64,
}

impl Eq for Handle {}

impl PartialEq for Handle {
    fn eq(&self, other: &Handle) -> bool {
        (self.dev, self.ino) == (other.dev, other.ino)
    }
}

impl AsRawFd for ::Handle {
    fn as_raw_fd(&self) -> RawFd {
        self.0.file.as_raw_fd()
    }
}

impl IntoRawFd for ::Handle {
    fn into_raw_fd(self) -> RawFd {
        self.0.file.into_raw_fd()
    }
}

impl Handle {
    pub fn from_path<P: AsRef<Path>>(p: P) -> io::Result<Handle> {
        Handle::from_file(try!(OpenOptions::new().read(true).open(p)))
    }

    pub fn from_file(file: File) -> io::Result<Handle> {
        let md = try!(file.metadata());
        Ok(Handle {
            file: file,
            dev: md.dev(),
            ino: md.ino(),
        })
    }

    pub fn stdin() -> io::Result<Handle> {
        Handle::from_file(unsafe { File::from_raw_fd(libc::STDIN_FILENO) })
    }

    pub fn stdout() -> io::Result<Handle> {
        Handle::from_file(unsafe { File::from_raw_fd(libc::STDOUT_FILENO) })
    }

    pub fn stderr() -> io::Result<Handle> {
        Handle::from_file(unsafe { File::from_raw_fd(libc::STDERR_FILENO) })
    }

    pub fn as_file(&self) -> &File {
        &self.file
    }

    pub fn as_file_mut(&mut self) -> &mut File {
        &mut self.file
    }
}
