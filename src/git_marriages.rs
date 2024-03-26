use crate::core::Mob;
use crate::git;
use crate::git::Tallies;
use std::collections::BTreeMap;

pub type MainResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
}

pub fn process<F>(tallies: F) -> Output
where
    F: Fn() -> git::Result<Tallies>,
{
    let mut results: BTreeMap<usize, Mob> = BTreeMap::new();
    for (mob, count) in tallies().unwrap() {
        results.insert(count, mob);
    }

    let message = results
        .iter()
        .fold("".to_owned(), |acc, (count, mob)| match mob.len() {
            1 => {
                let soloist = mob.first().unwrap();
                acc + &format!("{}: {} <{}> (solo)\n", count, &soloist.name, &soloist.email)
            }
            _ => {
                let mut authors = Vec::from_iter(mob);
                authors.sort();
                let names: Vec<String> = authors
                    .iter()
                    .map(|a| format!("{} <{}>", a.name, a.email))
                    .collect();
                let joined: String = names.join(", ");
                acc + &format!("{}: {}\n", count, joined)
            }
        });

    Output { message }
}

#[cfg(test)]
mod tests;
