use std::collections::HashSet;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

use serde::Deserialize;

pub type InputMob = BTreeSet<String>;
pub type Mob = HashSet<Author>;
pub type Org = BTreeMap<String, Team>;
pub type Team = BTreeMap<String, Author>;

#[derive(Clone, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
}

pub fn trailers(team: &Team, initials: &InputMob) -> String {
    initials
        .iter()
        .fold(String::new(), |acc, initial| match team.get(initial) {
            Some(coauthor) => {
                format!(
                    "{}Co-authored-by: {} <{}>\n",
                    acc, coauthor.name, coauthor.email
                )
            }
            None => acc,
        })
}

pub fn whole_org_as_team(org: &Org) -> Team {
    org.values().fold(Team::new(), |acc, team| {
        acc.into_iter().chain(team.to_owned()).collect()
    })
}

pub fn open_read_write(path: PathBuf) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
}

pub fn write_file(mut file: &File, contents: &str) -> io::Result<()> {
    file.set_len(0)?;
    file.rewind()?;
    file.write_all(contents.as_bytes())
}
