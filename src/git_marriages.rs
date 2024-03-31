use crate::config;
use crate::git;
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

pub fn process<F>(org: config::Org, tallies: F) -> Result<Output>
where
    F: Fn() -> git::Result<git::Tallies>,
{
    let team = config::whole_org_as_team(&org);
    let configured_authors: BTreeSet<config::Author> =
        team.values().map(|a| a.to_owned()).collect();

    let mut consolidated_tallies = git::Tallies::new();
    for (mob, count) in tallies()? {
        let mapped_mob: BTreeSet<config::Author> = mob
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

fn results(consolidated_tallies: git::Tallies) -> Vec<(usize, BTreeSet<config::Author>)> {
    let mut results: Vec<(usize, config::Mob)> = Vec::new();
    for (mob, count) in consolidated_tallies {
        results.push((count, mob));
    }
    results.sort();
    results
}

fn format_authors(authors: Vec<&crate::config::Author>) -> Vec<String> {
    authors.iter().map(|a| a.to_string()).collect()
}

fn solo_indicator(mob: &std::collections::BTreeSet<crate::config::Author>) -> &str {
    if mob.len() == 1 {
        " (solo)"
    } else {
        ""
    }
}

#[cfg(test)]
mod tests;
