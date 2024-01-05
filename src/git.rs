use git2::Repository;

pub fn head(dir: &str) -> Option<String> {
    match Repository::open(dir) {
        Ok(repo) => match repo.head() {
            Ok(h) => h.shorthand().map(|s| s.to_string()),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests;
