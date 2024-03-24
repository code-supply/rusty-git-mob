use crate::core::Author;
use git2::Error;
use git2::Oid;
use git2::Repository;
use regex::Regex;
use std::collections::HashSet;

pub fn commit_mob(dir: &str, oid: Oid) -> Result<HashSet<Author>, Error> {
    let repo = Repository::open(dir)?;
    let commit = repo.find_commit(oid)?;

    let pattern = Regex::new(r"(?i)co-authored-by: (.+) <(.+)>$").unwrap();
    let message = commit.message().expect("Couldn't get message");
    let matches: Vec<_> = pattern.captures_iter(message).collect();

    let author = commit.author();
    let mut authors = HashSet::from([Author {
        name: author.name().expect("couldn't get author name").to_owned(),
        email: author
            .email()
            .expect("couldn't get author email")
            .to_owned(),
    }]);

    for capture in matches {
        let name = capture.get(1).expect("Couldn't get name");
        let email = capture.get(2).expect("Couldn't get email");
        authors.insert(Author {
            name: name.as_str().to_owned(),
            email: email.as_str().to_owned(),
        });
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
