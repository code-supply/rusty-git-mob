use std::fs::{File, OpenOptions};
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

pub fn open_read_write(path: PathBuf) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
}

pub fn write_file(mut file: &File, contents: &str) -> std::io::Result<()> {
    file.set_len(0)?;
    file.rewind()?;
    file.write_all(contents.as_bytes())
}
