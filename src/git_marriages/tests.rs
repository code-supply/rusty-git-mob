use super::*;
use crate::core::Author;
use crate::core::Mob;

#[test]
fn can_show_mob_tallies_for_mobs_and_soloists() {
    let output = process(|| {
        let mut tallies = Tallies::new();
        tallies.insert(
            Mob::from([
                Author {
                    name: "Neil Young".to_owned(),
                    email: "neil@example.com".to_owned(),
                },
                Author {
                    name: "Andrew Bruce".to_owned(),
                    email: "me@andrewbruce.net".to_owned(),
                },
            ]),
            11,
        );
        tallies.insert(
            Mob::from([Author {
                name: "Andrew Bruce".to_owned(),
                email: "me@andrewbruce.net".to_owned(),
            }]),
            25,
        );
        tallies.insert(
            Mob::from([Author {
                name: "Billy Talbot".to_owned(),
                email: "billy@example.com".to_owned(),
            }]),
            25,
        );
        Ok(tallies)
    });
    assert_eq!(
        output.message,
        "11: Andrew Bruce <me@andrewbruce.net>, Neil Young <neil@example.com>\n\
         25: Andrew Bruce <me@andrewbruce.net> (solo)\n\
         25: Billy Talbot <billy@example.com> (solo)\n"
    );
}

#[test]
fn can_show_mob_tallies_for_soloists() {
    let output = process(|| {
        let mut tallies = Tallies::new();
        tallies.insert(
            Mob::from([Author {
                name: "Andrew Bruce".to_owned(),
                email: "me@andrewbruce.net".to_owned(),
            }]),
            25,
        );
        Ok(tallies)
    });
    assert_eq!(
        output.message,
        "25: Andrew Bruce <me@andrewbruce.net> (solo)\n"
    );
}
