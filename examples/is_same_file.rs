extern crate same_file;

use std::io;
use same_file::is_same_file;

fn try_main() -> Result<(), io::Error> {
    assert!(is_same_file("/bin/sh", "/usr/bin/sh")?);
    Ok(())
}

fn main() {
    try_main().unwrap();
}
