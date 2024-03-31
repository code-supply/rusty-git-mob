use crate::config::Author;
use crate::config::InputMob;
use crate::config::Team;

pub fn team(initials: &InputMob) -> Team {
    Team::from([
        (
            "ab".to_owned(),
            Author::new("Andrew Bruce", "me@andrewbruce.net"),
        ),
        (
            "fb".to_owned(),
            Author::new("Fred Brookes", "fred@example.com"),
        ),
    ])
    .into_iter()
    .filter(|(k, _v)| initials.contains(k))
    .collect()
}
