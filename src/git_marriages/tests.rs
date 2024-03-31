use super::*;
use crate::config::Author;
use crate::config::MainResult;
use crate::config::Mob;
use crate::config::Org;
use crate::config::Team;
use std::collections::BTreeSet;

#[test]
fn can_show_mob_tallies_for_mobs_and_soloists() -> MainResult {
    let org = Org::from([(
        "cool gang".to_owned(),
        Team::from([
            (
                "ab".to_owned(),
                Author::new_with_alternates(
                    "Andrew Bruce",
                    "me@andrewbruce.net",
                    BTreeSet::from(["andrew.bruce@maersk.com".to_owned()]),
                ),
            ),
            (
                "fb".to_owned(),
                Author::new("Random Person", "notincommits@example.com"),
            ),
        ]),
    )]);
    let output = process(org, || {
        let mut tallies = Tallies::new();
        tallies.insert(
            Mob::from([
                Author::new("Neil Young", "neil-not-on-team@example.com"),
                Author::new("Andrew Bruce", "me@andrewbruce.net"),
            ]),
            11,
        );
        tallies.insert(
            Mob::from([Author::new("Andrew Bruce", "andrew.bruce@maersk.com")]),
            25,
        );
        tallies.insert(
            Mob::from([Author::new("Andrew Bruce", "me@andrewbruce.net")]),
            26,
        );
        tallies.insert(
            Mob::from([Author::new("Billy Talbot", "billy@example.com")]),
            25,
        );
        Ok(tallies)
    });
    assert_eq!(
        output.unwrap().message,
        "11: Andrew Bruce <me@andrewbruce.net>, Neil Young <neil-not-on-team@example.com>\n\
         25: Billy Talbot <billy@example.com> (solo)\n\
         51: Andrew Bruce <me@andrewbruce.net> (solo)\n"
    );
    Ok(())
}

#[test]
fn copes_with_error_getting_tallies() -> MainResult {
    let org = Org::from([]);
    let output = process(org, || {
        Err(git::Error {
            message: "bad stuff happened".to_owned(),
        })
    });
    assert!(output.is_err());
    Ok(())
}
