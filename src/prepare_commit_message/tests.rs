use super::*;
use crate::output::Output;

#[test]
fn empty_coauthors_produces_empty_message() {
    assert_eq!(
        prepare_commit_message(&MobData::default(), "".to_owned(), Some("main")),
        Output::default()
    );
}

#[test]
fn empty_coauthors_and_only_comments_has_no_leading_whitespace() {
    assert_eq!(
        prepare_commit_message(
            &MobData::default(),
            "# original comment".to_owned(),
            Some("main")
        ),
        Output {
            message: "# original comment".to_owned()
        }
    );
}

#[test]
fn adds_preformatted_trailer_to_message_without_comments() {
    assert_eq!(
        prepare_commit_message(
            &MobData {
                current_mob_initials: CurrentMobInitials::default(),
                message: "some\nlines\n".to_owned(),
            },
            "Hello, World!".to_owned(),
            Some("main")
        ),
        Output {
            message: r#"Hello, World!

some
lines
"#
            .to_owned()
        }
    )
}

#[test]
fn adds_preformatted_trailers_to_existing_message() {
    assert_eq!(
        prepare_commit_message(
            &MobData {
                current_mob_initials: CurrentMobInitials::default(),
                message: "some\nlines\n".to_owned(),
            },
            r#"Hello, World!

# some comments
# go here
"#
            .to_owned(),
            Some("main")
        ),
        Output {
            message: r#"Hello, World!

some
lines

# some comments
# go here
"#
            .to_owned()
        }
    )
}

#[test]
fn adds_newline_and_preformatted_trailers_to_a_comment_only_message() {
    assert_eq!(
        prepare_commit_message(
            &MobData {
                current_mob_initials: CurrentMobInitials::default(),
                message: "some\nlines\n".to_owned(),
            },
            r#"# some comments
# go here
"#
            .to_owned(),
            Some("main")
        ),
        Output {
            message: r#"
some
lines

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
            &MobData {
                current_mob_initials: CurrentMobInitials::default(),
                message: "shouldn't\nsee\nme\n".to_owned(),
            },
            message.clone(),
            Some("main")
        ),
        Output { message }
    )
}

#[test]
fn does_not_include_additional_message_if_already_present_in_commit() {
    let message = r#"I'm a commit that's bound to be amended

story-1234

Some more text
"#
    .to_owned();
    assert_eq!(
        prepare_commit_message(
            &MobData {
                current_mob_initials: CurrentMobInitials::default(),
                message: "story-1234".to_owned(),
            },
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
            &MobData {
                current_mob_initials: CurrentMobInitials::default(),
                message: "shouldn't\nsee\nme".to_owned(),
            },
            message.clone(),
            None
        ),
        Output { message }
    )
}
