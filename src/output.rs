use crate::config;

pub type MainResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
}

pub fn trailers(team: &config::Team, initials: &config::CurrentMobInitials) -> String {
    initials
        .iter()
        .filter_map(|initial| team.get(initial))
        .map(|author| format!("Co-authored-by: {}\n", author))
        .collect()
}
