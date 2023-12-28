use super::*;

#[test]
fn empty_coauthors_produces_empty_message() {
    assert_eq!(
        prepare_commit_message(
            &Coauthors::default(),
            &Mob::default(),
            "".to_string(),
            Some("main")
        ),
        PrepareCommitMessageOutput::default()
    );
}

#[test]
fn empty_coauthors_and_only_comments_has_no_leading_whitespace() {
    assert_eq!(
        prepare_commit_message(
            &Coauthors::default(),
            &Mob::default(),
            "# original comment".to_string(),
            Some("main")
        ),
        PrepareCommitMessageOutput {
            message: "# original comment".to_string()
        }
    );
}

#[test]
fn adds_coauthors_to_message_without_comments() {
    assert_eq!(
        prepare_commit_message(
            &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
            &Mob::from(["ab".to_string(), "fb".to_string()]),
            "Hello, World!".to_string(),
            Some("main")
        ),
        PrepareCommitMessageOutput {
            message: r#"Hello, World!

Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>
"#
            .to_string()
        }
    )
}

#[test]
fn adds_coauthors_to_existing_message() {
    assert_eq!(
        prepare_commit_message(
            &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
            &Mob::from(["ab".to_string(), "fb".to_string()]),
            r#"Hello, World!

# some comments
# go here
"#
            .to_string(),
            Some("main")
        ),
        PrepareCommitMessageOutput {
            message: r#"Hello, World!

Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>

# some comments
# go here
"#
            .to_string()
        }
    )
}

#[test]
fn adds_newline_and_coauthors_to_a_comment_only_message() {
    assert_eq!(
        prepare_commit_message(
            &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
            &Mob::from(["ab".to_string(), "fb".to_string()]),
            r#"# some comments
# go here
"#
            .to_string(),
            Some("main")
        ),
        PrepareCommitMessageOutput {
            message: r#"
Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>

# some comments
# go here
"#
            .to_string()
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
    .to_string();
    assert_eq!(
        prepare_commit_message(
            &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
            &Mob::from(["ab".to_string(), "fb".to_string()]),
            message.clone(),
            Some("main")
        ),
        PrepareCommitMessageOutput { message }
    )
}

#[test]
fn does_not_change_commits_during_a_rebase() {
    let message = "I'm a commit without trailers".to_string();

    assert_eq!(
        prepare_commit_message(
            &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
            &Mob::from(["ab".to_string(), "fb".to_string()]),
            message.clone(),
            None
        ),
        PrepareCommitMessageOutput { message }
    )
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
