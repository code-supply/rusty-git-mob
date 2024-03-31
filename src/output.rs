use crate::config;

pub type MainResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
}

pub fn trailers(team: &config::Team, mob_initials: &config::CurrentMobInitials) -> String {
    mob_initials
        .iter()
        .filter_map(|initials| team.get(initials))
        .map(|author| format!("Co-authored-by: {}\n", author))
        .collect()
}
