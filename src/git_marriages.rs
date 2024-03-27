use crate::core::Mob;
use crate::git;
use crate::git::Tallies;

pub type MainResult = std::result::Result<(), Error>;

type Result<T> = std::result::Result<T, Error>;

impl From<git::Error> for Error {
    fn from(e: git::Error) -> Self {
        Error { message: e.message }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Error {
    pub message: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
}

pub fn process<F>(tallies: F) -> Result<Output>
where
    F: Fn() -> git::Result<Tallies>,
{
    let mut results: Vec<(usize, Mob)> = Vec::new();
    for (mob, count) in tallies()? {
        results.push((count, mob));
    }
    results.sort();

    let message = results.iter().fold("".to_owned(), |acc, (count, mob)| {
        let mut authors = Vec::from_iter(mob);
        authors.sort();
        let authors_formatted: String = format_authors(authors).join(", ");
        acc + &format!("{}: {}{}\n", count, authors_formatted, solo_indicator(mob))
    });

    Ok(Output { message })
}

fn format_authors(authors: Vec<&crate::core::Author>) -> Vec<String> {
    authors.iter().map(|a| a.to_string()).collect()
}

fn solo_indicator(mob: &std::collections::BTreeSet<crate::core::Author>) -> &str {
    match mob.len() {
        1 => " (solo)",
        _ => "",
    }
}

#[cfg(test)]
mod tests;
