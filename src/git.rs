use crate::config::Author;
use crate::config::Mob;
use git2::Oid;
use git2::Repository;
use regex::Regex;
use std::collections::HashMap;

pub type Tallies = HashMap<Mob, usize>;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub message: String,
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        Error {
            message: e.message().to_owned(),
        }
    }
}

pub fn mob_tally(dir: &str) -> Result<Tallies> {
    let repo = Repository::open(dir)?;
    let mut revwalk = repo.revwalk()?;

    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut tallies = Tallies::new();
    for commit_id in revwalk {
        let mob = commit_mob(dir, commit_id?)?;
        let count = tallies.get(&mob).unwrap_or(&0) + 1;
        tallies.insert(mob, count);
    }
    Ok(tallies)
}

pub fn commit_mob(dir: &str, oid: Oid) -> Result<Mob> {
    let pattern = Regex::new(r"(?i)co-authored-by: (?<name>.+) <(?<email>.+)>").unwrap();
    let repo = Repository::open(dir)?;
    let commit = repo.find_commit(oid)?;
    let message = commit.message().expect("Message should be valid UTF-8");
    let author = commit.author();
    let mut authors = Mob::from([Author::new(
        author.name().expect("Name should be valid UTF-8"),
        author.email().expect("Email should be valid UTF-8"),
    )]);

    for captures in pattern.captures_iter(message) {
        authors.insert(Author::new(&captures["name"], &captures["email"]));
    }

    Ok(authors)
}

pub fn head(dir: &str) -> Option<String> {
    match Repository::open(dir) {
        Ok(repo) => match repo.head() {
            Ok(h) => h.shorthand().map(str::to_owned),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests;
