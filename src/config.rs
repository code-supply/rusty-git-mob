use std::collections::{BTreeMap, BTreeSet};

mod author;
pub use author::Author;

pub type CurrentMobInitials = BTreeSet<String>;
pub type Mob = BTreeSet<Author>;
pub type Org = BTreeMap<String, Team>;
pub type Team = BTreeMap<String, Author>;

pub fn whole_org_as_team(org: &Org) -> Team {
    org.values().fold(Team::new(), |acc, team| {
        acc.into_iter().chain(team.to_owned()).collect()
    })
}
