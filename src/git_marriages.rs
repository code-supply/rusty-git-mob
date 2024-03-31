use crate::core::whole_org_as_team;
use crate::core::Author;
use crate::core::Mob;
use crate::core::Org;
use crate::git;
use crate::git::Tallies;
use std::collections::BTreeSet;

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

pub fn process<F>(org: Org, tallies: F) -> Result<Output>
where
    F: Fn() -> git::Result<Tallies>,
{
    let team = whole_org_as_team(&org);
    let configured_authors: BTreeSet<Author> = team.values().map(|a| a.to_owned()).collect();

    let mut consolidated_tallies = Tallies::new();
    for (mob, count) in tallies()? {
        let mapped_mob: BTreeSet<Author> = mob
            .iter()
            .map(|author| author.use_configured(&configured_authors))
            .collect();

        let total = consolidated_tallies.get(&mapped_mob).unwrap_or(&0usize) + count;

        consolidated_tallies.insert(mapped_mob, total);
    }

    let message = results(consolidated_tallies)
        .iter()
        .fold("".to_owned(), |acc, (count, mob)| {
            let mut authors = Vec::from_iter(mob);
            authors.sort();
            let authors_formatted: String = format_authors(authors).join(", ");
            acc + &format!("{}: {}{}\n", count, authors_formatted, solo_indicator(mob))
        });

    Ok(Output { message })
}

fn results(consolidated_tallies: Tallies) -> Vec<(usize, BTreeSet<Author>)> {
    let mut results: Vec<(usize, Mob)> = Vec::new();
    for (mob, count) in consolidated_tallies {
        results.push((count, mob));
    }
    results.sort();
    results
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
