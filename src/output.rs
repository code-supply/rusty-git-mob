use crate::config::*;

pub type MainResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
}

pub fn trailers(team: &Team, initials: &InputMob) -> String {
    initials
        .iter()
        .fold(String::new(), |acc, initial| match team.get(initial) {
            Some(author) => {
                format!("{}Co-authored-by: {}\n", acc, author)
            }
            None => acc,
        })
}
