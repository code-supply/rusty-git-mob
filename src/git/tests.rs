use super::*;
use crate::core::Author;
use crate::core::Mob;
use std::collections::HashSet;

#[test]
fn can_get_mob_tally_from_multiple_commits() {
    let dir = "tmp/mob-tally-multiple";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));

    initial_commit(
        &repo,
        "Initial commit
co-AuthoReD-By: Andrew Bruce <me@andrewbruce.net>",
    );
    commit(
        &repo,
        "Another commit
Co-authored-By: Maggie Hamilton <margaret@example.com>",
    );
    commit(
        &repo,
        "Yet another commit
co-authored-by: Andrew Bruce <me@andrewbruce.net>",
    );

    let tally = mob_tally(dir).expect("Couldn't get tally");
    let anne_and_andrew = Mob::from([
        Author {
            name: "Anne Other".to_owned(),
            email: "anne@example.com".to_owned(),
        },
        Author {
            name: "Andrew Bruce".to_owned(),
            email: "me@andrewbruce.net".to_owned(),
        },
    ]);
    let anne_and_maggie = Mob::from([
        Author {
            name: "Anne Other".to_owned(),
            email: "anne@example.com".to_owned(),
        },
        Author {
            name: "Maggie Hamilton".to_owned(),
            email: "margaret@example.com".to_owned(),
        },
    ]);

    assert_eq!(2, tally.len());
    assert_eq!(2, *tally.get(&anne_and_andrew).expect("Mob not in result"));
    assert_eq!(1, *tally.get(&anne_and_maggie).expect("Mob not in result"));
}

#[test]
fn can_get_mob_tally_from_commit_history_of_one() {
    let dir = "tmp/mob-tally";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));

    initial_commit(
        &repo,
        "Initial commit
co-AuthoReD-By: Andrew Bruce <me@andrewbruce.net>",
    );

    let tally = mob_tally(dir).expect("Couldn't get tally");
    let expected_mob = Mob::from([
        Author {
            name: "Anne Other".to_owned(),
            email: "anne@example.com".to_owned(),
        },
        Author {
            name: "Andrew Bruce".to_owned(),
            email: "me@andrewbruce.net".to_owned(),
        },
    ]);

    assert_eq!(1, tally.len());
    assert_eq!(1, *tally.get(&expected_mob).expect("Mob not in result"))
}

#[test]
fn can_get_mob_from_commit_with_trailers() {
    let dir = "tmp/authors-with-trailers";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));
    let oid = initial_commit(
        &repo,
        "Initial commit
co-autHoreD-By: nobod <y-par>seable
co-AuthoReD-By: Andrew Bruce <me@andrewbruce.net>",
    );

    assert_eq!(
        Ok(Mob::from([
            Author {
                name: "Anne Other".to_owned(),
                email: "anne@example.com".to_owned()
            },
            Author {
                name: "Andrew Bruce".to_owned(),
                email: "me@andrewbruce.net".to_owned()
            }
        ])),
        commit_mob(dir, oid)
    );
}

#[test]
fn can_get_mob_from_commit_without_trailers() {
    let dir = "tmp/authors-no-trailers";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));
    let oid = initial_commit(&repo, "Initial commit");

    assert_eq!(
        Ok(Mob::from([Author {
            name: "Anne Other".to_owned(),
            email: "anne@example.com".to_owned()
        }])),
        commit_mob(dir, oid)
    );
}

#[test]
fn head_of_non_repository_is_none() {
    assert_eq!(head("/tmp"), None);
}

#[test]
fn head_of_one_commit_is_a_short_string() {
    let dir = "tmp/my-fixture-2";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));
    initial_commit(&repo, "Initial commit");

    assert!(
        HashSet::from([Some("master".to_owned()), Some("main".to_owned())]).contains(&head(dir))
    );
}

#[test]
fn head_of_no_commits_is_none() {
    let dir = "tmp/my-fixture";
    repo(dir, "");
    assert_eq!(head(dir), None);
}

fn repo(dir: &str, config: &str) -> Repository {
    let _maybe_remove = std::fs::remove_dir_all(dir);
    let repo = Repository::init(dir).expect("couldn't make fixture repo");
    std::fs::write(dir.to_owned() + "/.git/config", config).expect("couldn't write config");
    repo
}

fn initial_commit(repo: &Repository, message: &str) -> Oid {
    let tree_id = {
        let mut index = repo.index().expect("couldn't get index");
        index.write_tree().expect("couldn't write tree")
    };

    let tree = repo.find_tree(tree_id).expect("couldn't find tree");

    let sig = repo.signature().expect("couldn't make signature");
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[])
        .expect("couldn't commit")
}

fn commit(repo: &Repository, message: &str) -> Oid {
    let head = repo.head().expect("Couldn't get HEAD");
    let oid = head.target().expect("Couldn't get target");
    let commit = repo.find_commit(oid).expect("Couldn't get latest commit");

    let tree_id = {
        let mut index = repo.index().expect("couldn't get index");
        index.write_tree().expect("couldn't write tree")
    };

    let tree = repo.find_tree(tree_id).expect("couldn't find tree");

    let sig = repo.signature().expect("couldn't make signature");
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&commit])
        .expect("couldn't commit")
}

fn committer_config(name: &str, email: &str) -> String {
    format!(
        r#"[user]
        name = "{}"
        email = "{}"#,
        name, email
    )
}
