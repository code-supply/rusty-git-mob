use super::*;
use crate::core::Author;
use crate::git_mob::Output;

#[test]
fn empty_input_returns_empty_output() {
    assert_eq!(
        process(
            &team(&InputMob::new()),
            &InputMob::new(),
            &Args {
                initials: vec![],
                ..Default::default()
            }
        ),
        Output::default()
    );
}

#[test]
fn forming_a_mob_outputs_the_mob_and_template() {
    assert_eq!(
        process(
            &team(&InputMob::from(["ab".to_owned()])),
            &InputMob::new(),
            &Args {
                initials: vec!["ab".to_owned()],
                ..Default::default()
            }
        ),
        Output {
            message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
            template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
            mob: InputMob::from(["ab".to_owned()]),
        }
    );
}

#[test]
fn can_add_many_mobsters() {
    assert_eq!(
        process(
            &team(&InputMob::from(["ab".to_owned(), "fb".to_owned()])),
            &InputMob::new(),
            &Args {
                initials: vec!["ab".to_owned(), "fb".to_owned()],
                ..Default::default()
            }
        ),
        Output {
            message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                .to_owned(),
            template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                .to_owned(),
            mob: InputMob::from(["ab".to_owned(), "fb".to_owned()]),
        }
    );
}

#[test]
fn calling_without_initials_outputs_current_mob() {
    assert_eq!(
        process(
            &team(&InputMob::from(["ab".to_owned(), "fb".to_owned()])),
            &InputMob::from(["ab".to_owned(), "fb".to_owned()]),
            &Args {
                initials: vec![],
                ..Default::default()
            },
        ),
        Output {
            message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                .to_owned(),
            template: "

Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                .to_owned(),
            mob: InputMob::from(["ab".to_owned(), "fb".to_owned()]),
        }
    )
}

#[test]
fn coauthors_are_sorted_by_initials() {
    let a = team(&InputMob::from(["ab".to_owned(), "fb".to_owned()]));
    let expected: Vec<_> = a.iter().collect();
    let b = team(&InputMob::from(["fb".to_owned(), "ab".to_owned()]));
    let actual: Vec<_> = b.iter().collect();
    assert_eq!(actual, expected)
}

#[test]
fn mob_is_sorted() {
    let a = InputMob::from(["ab".to_owned(), "fb".to_owned()]);
    let expected: Vec<_> = a.iter().collect();
    let b = InputMob::from(["fb".to_owned(), "ab".to_owned()]);
    let actual: Vec<_> = b.iter().collect();
    assert_eq!(actual, expected)
}

fn team(initials: &InputMob) -> Team {
    Team::from([
        (
            "ab".to_owned(),
            Author {
                name: "Andrew Bruce".to_owned(),
                email: "me@andrewbruce.net".to_owned(),
            },
        ),
        (
            "fb".to_owned(),
            Author {
                name: "Fred Brookes".to_owned(),
                email: "fred@example.com".to_owned(),
            },
        ),
    ])
    .into_iter()
    .filter(|(k, _v)| initials.contains(k))
    .collect()
}
