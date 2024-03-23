use super::*;
use crate::core::Coauthor;
use crate::git_mob::Output;

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
        Output::default()
    );
}

#[test]
fn forming_a_mob_outputs_the_mob_and_template() {
    assert_eq!(
        process(
            &coauthors(&Mob::from(["ab".to_owned()])),
            &Mob::new(),
            &Args {
                initials: vec!["ab".to_owned()],
                ..Default::default()
            }
        ),
        Output {
            message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
            template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
            mob: Mob::from(["ab".to_owned()]),
        }
    );
}

#[test]
fn can_add_many_mobsters() {
    assert_eq!(
        process(
            &coauthors(&Mob::from(["ab".to_owned(), "fb".to_owned()])),
            &Mob::new(),
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
            mob: Mob::from(["ab".to_owned(), "fb".to_owned()]),
        }
    );
}

#[test]
fn calling_without_initials_outputs_current_mob() {
    assert_eq!(
        process(
            &coauthors(&Mob::from(["ab".to_owned(), "fb".to_owned()])),
            &Mob::from(["ab".to_owned(), "fb".to_owned()]),
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
            mob: Mob::from(["ab".to_owned(), "fb".to_owned()]),
        }
    )
}

#[test]
fn coauthors_are_sorted_by_initials() {
    let a = coauthors(&Mob::from(["ab".to_owned(), "fb".to_owned()]));
    let expected: Vec<_> = a.iter().collect();
    let b = coauthors(&Mob::from(["fb".to_owned(), "ab".to_owned()]));
    let actual: Vec<_> = b.iter().collect();
    assert_eq!(actual, expected)
}

#[test]
fn mob_is_sorted() {
    let a = Mob::from(["ab".to_owned(), "fb".to_owned()]);
    let expected: Vec<_> = a.iter().collect();
    let b = Mob::from(["fb".to_owned(), "ab".to_owned()]);
    let actual: Vec<_> = b.iter().collect();
    assert_eq!(actual, expected)
}

fn coauthors(initials: &Mob) -> Coauthors {
    Coauthors::from([
        (
            "ab".to_owned(),
            Coauthor {
                name: "Andrew Bruce".to_owned(),
                email: "me@andrewbruce.net".to_owned(),
            },
        ),
        (
            "fb".to_owned(),
            Coauthor {
                name: "Fred Brookes".to_owned(),
                email: "fred@example.com".to_owned(),
            },
        ),
    ])
    .into_iter()
    .filter(|(k, _v)| initials.contains(k))
    .collect()
}
