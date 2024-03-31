use std::collections::BTreeSet;

use super::*;
use crate::output;

#[test]
fn can_show_mob_tallies_for_mobs_and_soloists() -> output::MainResult {
    let org = config::Org::from([(
        "cool gang".to_owned(),
        config::Team::from([
            (
                "ab".to_owned(),
                config::Author::new_with_alternates(
                    "Andrew Bruce",
                    "me@andrewbruce.net",
                    BTreeSet::from(["andrew.bruce@maersk.com".to_owned()]),
                ),
            ),
            (
                "fb".to_owned(),
                config::Author::new("Random Person", "notincommits@example.com"),
            ),
        ]),
    )]);
    let output = process(org, || {
        let mut tallies = git::Tallies::new();
        tallies.insert(
            config::Mob::from([
                config::Author::new("Neil Young", "neil-not-on-team@example.com"),
                config::Author::new("Andrew Bruce", "me@andrewbruce.net"),
            ]),
            11,
        );
        tallies.insert(
            config::Mob::from([config::Author::new(
                "Andrew Bruce",
                "andrew.bruce@maersk.com",
            )]),
            25,
        );
        tallies.insert(
            config::Mob::from([config::Author::new("Andrew Bruce", "me@andrewbruce.net")]),
            26,
        );
        tallies.insert(
            config::Mob::from([config::Author::new("Billy Talbot", "billy@example.com")]),
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
fn copes_with_error_getting_tallies() -> output::MainResult {
    let org = config::Org::from([]);
    let output = process(org, || {
        Err(git::Error {
            message: "bad stuff happened".to_owned(),
        })
    });
    assert!(output.is_err());
    Ok(())
}
