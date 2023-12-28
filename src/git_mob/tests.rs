use super::*;
use crate::core::Coauthor;
use crate::git_mob::{self, Output};

#[test]
fn empty_input_returns_empty_output() {
    assert_eq!(
        process(
            &coauthors(&Mob::new()),
            &Mob::new(),
            &Args {
                initials: vec![],
                ..Default::default()
            }
        ),
        git_mob::Output::default()
    );
}

#[test]
fn forming_a_mob_outputs_the_mob_and_template() {
    assert_eq!(
        process(
            &coauthors(&Mob::from(["ab".to_string()])),
            &Mob::new(),
            &Args {
                initials: vec!["ab".to_string()],
                ..Default::default()
            }
        ),
        Output {
            message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
            template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
            mob: Mob::from(["ab".to_string()]),
        }
    );
}

#[test]
fn can_add_many_mobsters() {
    assert_eq!(
        process(
            &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
            &Mob::new(),
            &Args {
                initials: vec!["ab".to_string(), "fb".to_string()],
                ..Default::default()
            }
        ),
        Output {
            message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                .to_string(),
            template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                .to_string(),
            mob: Mob::from(["ab".to_string(), "fb".to_string()]),
        }
    );
}

#[test]
fn calling_without_initials_outputs_current_mob() {
    assert_eq!(
        process(
            &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
            &Mob::from(["ab".to_string(), "fb".to_string()]),
            &Args {
                initials: vec![],
                ..Default::default()
            },
        ),
        Output {
            message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                .to_string(),
            template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                .to_string(),
            mob: Mob::from(["ab".to_string(), "fb".to_string()]),
        }
    )
}

#[test]
fn soloing_shows_no_output_and_wipes_mob_and_template() {
    assert_eq!(
        process(
            &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
            &Mob::from(["ab".to_string(), "fb".to_string()]),
            &Args {
                initials: vec!["ab".to_string()],
                solo: true,
                pick: false
            },
        ),
        Output {
            message: "".to_string(),
            template: "".to_string(),
            mob: Mob::new(),
        }
    )
}

#[test]
fn coauthors_are_sorted_by_initials() {
    let a = coauthors(&Mob::from(["ab".to_string(), "fb".to_string()]));
    let expected: Vec<_> = a.iter().collect();
    let b = coauthors(&Mob::from(["fb".to_string(), "ab".to_string()]));
    let actual: Vec<_> = b.iter().collect();
    assert_eq!(actual, expected)
}

#[test]
fn mob_is_sorted() {
    let a = Mob::from(["ab".to_string(), "fb".to_string()]);
    let expected: Vec<_> = a.iter().collect();
    let b = Mob::from(["fb".to_string(), "ab".to_string()]);
    let actual: Vec<_> = b.iter().collect();
    assert_eq!(actual, expected)
}

fn coauthors(initials: &Mob) -> Coauthors {
    Coauthors::from([
        (
            "ab".to_string(),
            Coauthor {
                name: "Andrew Bruce".to_string(),
                email: "me@andrewbruce.net".to_string(),
            },
        ),
        (
            "fb".to_string(),
            Coauthor {
                name: "Fred Brookes".to_string(),
                email: "fred@example.com".to_string(),
            },
        ),
    ])
    .into_iter()
    .filter(|(k, _v)| initials.contains(k))
    .collect()
}
