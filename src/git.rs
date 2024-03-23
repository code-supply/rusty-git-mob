use crate::core::Coauthor;
use git2::Error;
use git2::Oid;
use git2::Repository;

pub fn authors(dir: &str, oid: Oid) -> Result<Vec<Coauthor>, Error> {
    let repo = Repository::open(dir)?;
    let commit = repo.find_commit(oid)?;
    let author = commit.author();
    Ok(vec![Coauthor {
        name: author.name().expect("couldn't get author name").to_owned(),
        email: author
            .email()
            .expect("couldn't get author email")
            .to_owned(),
    }])
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
