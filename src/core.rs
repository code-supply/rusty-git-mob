use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::{env, io};

pub type Mob = BTreeSet<String>;
pub type Coauthors = BTreeMap<String, Coauthor>;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CoauthorsConfig {
    pub coauthors: Coauthors,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Coauthor {
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct PrepareCommitMessageOutput {
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

pub fn resolve_path(env_var_name: &str, filename: &str) -> Result<PathBuf, String> {
    match env::var(env_var_name) {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_e) => match home::home_dir() {
            Some(path_buf) => Ok(path_buf.as_path().join(filename)),
            None => Err(format!(
                "{} not set and couldn't find your home dir!",
                env_var_name
            )),
        },
    }
}
