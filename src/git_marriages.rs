use crate::core::Mob;
use crate::git;

use std::collections::HashMap;

pub type MainResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
}

pub fn process<F>(tallies: F) -> Output
where
    F: Fn() -> git::Result<HashMap<Mob, usize>>,
{
    let message = tallies()
        .unwrap()
        .iter()
        .fold("".to_owned(), |acc, (mob, count)| {
            let soloist = mob.first().unwrap();
            acc + &format!("{} (solo): {}", &soloist.name, count)
        });

    Output { message }
}

#[cfg(test)]
mod tests;
