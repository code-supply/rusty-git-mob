use git2::Repository;

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
