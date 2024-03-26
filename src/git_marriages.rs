use crate::git;
use crate::git::Tallies;

pub type MainResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
}

pub fn process<F>(tallies: F) -> Output
where
    F: Fn() -> git::Result<Tallies>,
{
    let message = tallies()
        .unwrap()
        .iter()
        .fold("".to_owned(), |acc, (mob, count)| match mob.len() {
            1 => {
                let soloist = mob.first().unwrap();
                acc + &format!("{} (solo): {}\n", &soloist.name, count)
            }
            _ => {
                let mut authors = Vec::from_iter(mob);
                authors.sort();
                let names: Vec<String> = authors.iter().map(|a| a.name.clone()).collect();
                let joined: String = names.join(", ");
                acc + &format!("{}: {}\n", joined, count)
            }
        });

    Output { message }
}

#[cfg(test)]
mod tests;
