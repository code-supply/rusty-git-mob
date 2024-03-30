use super::*;

#[test]
fn empty_coauthors_produces_empty_message() {
    assert_eq!(
        prepare_commit_message(
            &Org::from([("cool gang".to_owned(), Team::default())]),
            &InputMob::default(),
            "".to_owned(),
            Some("main")
        ),
        Output::default()
    );
}

#[test]
fn empty_coauthors_and_only_comments_has_no_leading_whitespace() {
    assert_eq!(
        prepare_commit_message(
            &Org::from([("cool gang".to_owned(), Team::default())]),
            &InputMob::default(),
            "# original comment".to_owned(),
            Some("main")
        ),
        Output {
            message: "# original comment".to_owned()
        }
    );
}

#[test]
fn adds_coauthors_to_message_without_comments() {
    assert_eq!(
        prepare_commit_message(
            &Org::from([(
                "cool gang".to_owned(),
                coauthors(&InputMob::from(["ab".to_owned(), "fb".to_owned()])),
            )]),
            &InputMob::from(["ab".to_owned(), "fb".to_owned()]),
            "Hello, World!".to_owned(),
            Some("main")
        ),
        Output {
            message: r#"Hello, World!

Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>
"#
            .to_owned()
        }
    )
}

#[test]
fn adds_coauthors_to_existing_message() {
    assert_eq!(
        prepare_commit_message(
            &Org::from([(
                "cool gang".to_owned(),
                coauthors(&InputMob::from(["ab".to_owned(), "fb".to_owned()])),
            )]),
            &InputMob::from(["ab".to_owned(), "fb".to_owned()]),
            r#"Hello, World!

# some comments
# go here
"#
            .to_owned(),
            Some("main")
        ),
        Output {
            message: r#"Hello, World!

Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>

# some comments
# go here
"#
            .to_owned()
        }
    )
}

#[test]
fn adds_newline_and_coauthors_to_a_comment_only_message() {
    assert_eq!(
        prepare_commit_message(
            &Org::from([(
                "cool gang".to_owned(),
                coauthors(&InputMob::from(["ab".to_owned(), "fb".to_owned()])),
            )]),
            &InputMob::from(["ab".to_owned(), "fb".to_owned()]),
            r#"# some comments
# go here
"#
            .to_owned(),
            Some("main")
        ),
        Output {
            message: r#"
Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>

# some comments
# go here
"#
            .to_owned()
        }
    )
}

#[test]
fn preserves_existing_coauthors() {
    let message = r#"I'm a commit that's bound to be amended

cO-aUthoRed-by: Original Author <og@authors.biz>

# some comments
# go here
"#
    .to_owned();
    assert_eq!(
        prepare_commit_message(
            &Org::from([(
                "cool gang".to_owned(),
                coauthors(&InputMob::from(["ab".to_owned(), "fb".to_owned()])),
            )]),
            &InputMob::from(["ab".to_owned(), "fb".to_owned()]),
            message.clone(),
            Some("main")
        ),
        Output { message }
    )
}

#[test]
fn does_not_change_commits_during_a_rebase() {
    let message = "I'm a commit without trailers".to_owned();

    assert_eq!(
        prepare_commit_message(
            &Org::from([(
                "cool gang".to_owned(),
                coauthors(&InputMob::from(["ab".to_owned(), "fb".to_owned()]))
            )]),
            &InputMob::from(["ab".to_owned(), "fb".to_owned()]),
            message.clone(),
            None
        ),
        Output { message }
    )
}

fn coauthors(initials: &InputMob) -> Team {
    Team::from([
        (
            "ab".to_owned(),
            Author::new("Andrew Bruce".to_owned(), "me@andrewbruce.net".to_owned()),
        ),
        (
            "fb".to_owned(),
            Author::new("Fred Brookes".to_owned(), "fred@example.com".to_owned()),
        ),
    ])
    .into_iter()
    .filter(|(k, _v)| initials.contains(k))
    .collect()
}
