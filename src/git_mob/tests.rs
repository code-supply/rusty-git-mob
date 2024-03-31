use super::*;
use crate::git_mob::Output;
use crate::tests::team;

#[test]
fn empty_input_returns_empty_output() {
    assert_eq!(
        process(
            &team(&config::CurrentMobInitials::new()),
            &config::CurrentMobInitials::new(),
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
            &team(&config::CurrentMobInitials::from(["ab".to_owned()])),
            &config::CurrentMobInitials::new(),
            &Args {
                initials: vec!["ab".to_owned()],
                ..Default::default()
            }
        ),
        Output {
            message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
            template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
            mob: config::CurrentMobInitials::from(["ab".to_owned()]),
        }
    );
}

#[test]
fn can_add_many_mobsters() {
    assert_eq!(
        process(
            &team(&config::CurrentMobInitials::from([
                "ab".to_owned(),
                "fb".to_owned()
            ])),
            &config::CurrentMobInitials::new(),
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
            mob: config::CurrentMobInitials::from(["ab".to_owned(), "fb".to_owned()]),
        }
    );
}

#[test]
fn calling_without_initials_outputs_current_mob() {
    assert_eq!(
        process(
            &team(&config::CurrentMobInitials::from([
                "ab".to_owned(),
                "fb".to_owned()
            ])),
            &config::CurrentMobInitials::from(["ab".to_owned(), "fb".to_owned()]),
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
            mob: config::CurrentMobInitials::from(["ab".to_owned(), "fb".to_owned()]),
        }
    )
}

#[test]
fn coauthors_are_sorted_by_initials() {
    let a = team(&config::CurrentMobInitials::from([
        "ab".to_owned(),
        "fb".to_owned(),
    ]));
    let expected: Vec<_> = a.iter().collect();
    let b = team(&config::CurrentMobInitials::from([
        "fb".to_owned(),
        "ab".to_owned(),
    ]));
    let actual: Vec<_> = b.iter().collect();
    assert_eq!(actual, expected)
}

#[test]
fn mob_is_sorted() {
    let a = config::CurrentMobInitials::from(["ab".to_owned(), "fb".to_owned()]);
    let expected: Vec<_> = a.iter().collect();
    let b = config::CurrentMobInitials::from(["fb".to_owned(), "ab".to_owned()]);
    let actual: Vec<_> = b.iter().collect();
    assert_eq!(actual, expected)
}
