use super::*;
use crate::core::Coauthor;

use std::collections::HashSet;

#[test]
fn can_get_authors_of_commit_with_trailers() {
    let dir = "tmp/authors-with-trailers";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));
    let oid = commit(
        &repo,
        "Initial commit
co-autHoreD-By: nobod <y-par>seable
co-AuthoReD-By: Andrew Bruce <me@andrewbruce.net>",
    );

    assert_eq!(
        Ok(HashSet::from([
            Coauthor {
                name: "Anne Other".to_owned(),
                email: "anne@example.com".to_owned()
            },
            Coauthor {
                name: "Andrew Bruce".to_owned(),
                email: "me@andrewbruce.net".to_owned()
            }
        ])),
        commit_authors(dir, oid)
    );
}

#[test]
fn can_get_author_of_commit_without_trailers() {
    let dir = "tmp/authors-no-trailers";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));
    let oid = commit(&repo, "Initial commit");

    assert_eq!(
        Ok(HashSet::from([Coauthor {
            name: "Anne Other".to_owned(),
            email: "anne@example.com".to_owned()
        }])),
        commit_authors(dir, oid)
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
    commit(&repo, "Initial commit");

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

fn commit(repo: &Repository, message: &str) -> Oid {
    let sig = repo.signature().expect("couldn't make signature");
    let tree_id = {
        let mut index = repo.index().expect("couldn't get index");
        index.write_tree().expect("couldn't write tree")
    };

    let tree = repo.find_tree(tree_id).expect("couldn't find tree");

    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[])
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
