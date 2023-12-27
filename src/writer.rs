use std::fs::File;
use std::io;
use std::io::Seek;
use std::io::Write;

pub fn write_file(mut file: &File, contents: &str) -> io::Result<()> {
    file.set_len(0)?;
    file.rewind()?;
    file.write_all(contents.as_bytes())
}
