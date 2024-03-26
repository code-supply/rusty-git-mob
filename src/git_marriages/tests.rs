use super::*;
use crate::core::Author;

#[test]
fn can_show_mob_tallies_for_soloists() {
    let output = process(|| {
        let mut tallies = HashMap::new();
        tallies.insert(
            Mob::from([Author {
                name: "Andrew Bruce".to_owned(),
                email: "me@andrewbruce.net".to_owned(),
            }]),
            25,
        );
        Ok(tallies)
    });
    assert_eq!(output.message, "Andrew Bruce (solo): 25");
}
