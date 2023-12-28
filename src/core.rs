use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

use serde::Deserialize;

pub type Mob = BTreeSet<String>;
pub type Coauthors = BTreeMap<String, Coauthor>;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Coauthor {
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
}

pub fn trailers(coauthors: &Coauthors, initials: &Mob) -> String {
    initials
        .iter()
        .fold(String::new(), |acc, initial| match coauthors.get(initial) {
            Some(coauthor) => {
                format!(
                    "{}Co-authored-by: {} <{}>\n",
                    acc, coauthor.name, coauthor.email
                )
            }
            None => acc,
        })
}

pub fn open_read_write(path: PathBuf) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}

pub fn write_file(mut file: &File, contents: &str) -> io::Result<()> {
    file.set_len(0)?;
    file.rewind()?;
    file.write_all(contents.as_bytes())
}
