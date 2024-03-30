use super::*;
use crate::core::Author;
use crate::core::Mob;

#[test]
fn can_show_mob_tallies_for_mobs_and_soloists() -> MainResult {
    let output = process(|| {
        let mut tallies = Tallies::new();
        tallies.insert(
            Mob::from([
                Author::new("Neil Young", "neil@example.com"),
                Author::new("Andrew Bruce", "me@andrewbruce.net"),
            ]),
            11,
        );
        tallies.insert(
            Mob::from([Author::new("Andrew Bruce", "me@andrewbruce.net")]),
            25,
        );
        tallies.insert(
            Mob::from([Author::new("Billy Talbot", "billy@example.com")]),
            25,
        );
        Ok(tallies)
    });
    assert_eq!(
        output?.message,
        "11: Andrew Bruce <me@andrewbruce.net>, Neil Young <neil@example.com>\n\
         25: Andrew Bruce <me@andrewbruce.net> (solo)\n\
         25: Billy Talbot <billy@example.com> (solo)\n"
    );
    Ok(())
}

#[test]
fn copes_with_error_getting_tallies() -> MainResult {
    let output = process(|| {
        Err(git::Error {
            message: "bad stuff happened".to_owned(),
        })
    });
    assert!(output.is_err());
    Ok(())
}
