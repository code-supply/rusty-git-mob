use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

use serde::Deserialize;

pub type MainResult = Result<(), Box<dyn std::error::Error>>;

pub type InputMob = BTreeSet<String>;
pub type Mob = BTreeSet<Author>;
pub type Org = BTreeMap<String, Team>;
pub type Team = BTreeMap<String, Author>;

#[derive(Clone, Default, Deserialize, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct Author {
    pub name: String,
    email: String,
    alternate_emails: Option<BTreeSet<String>>,
}

impl Author {
    pub fn new(name: &str, email: &str) -> Author {
        Author {
            name: name.to_owned(),
            email: email.to_owned(),
            ..Default::default()
        }
    }

    pub fn new_with_alternates(
        name: &str,
        email: &str,
        alternate_emails: BTreeSet<String>,
    ) -> Author {
        Author {
            name: name.to_owned(),
            email: email.to_owned(),
            alternate_emails: Some(alternate_emails),
        }
    }

    pub fn is_configured_equivalent_of(&self, found_author: &Author) -> bool {
        self.email == found_author.email
            || self
                .alternate_emails
                .to_owned()
                .is_some_and(|e| e.contains(&found_author.email))
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
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
