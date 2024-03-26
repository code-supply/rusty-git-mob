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
        Ok(tallies)
    });
    assert_eq!(
        output.message,
        "Andrew Bruce (solo): 25\n\
         Andrew Bruce, Neil Young: 11\n"
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
    assert_eq!(output.message, "Andrew Bruce (solo): 25\n");
}
