use super::*;

use std::collections::HashSet;

#[test]
fn head_of_non_repository_is_none() {
    assert_eq!(head("/tmp"), None);
}

#[test]
fn head_of_one_commit_is_a_short_string() {
    let dir = "tmp/my-fixture-2";
    let _maybe_remove = std::fs::remove_dir_all(dir);
    let repo = Repository::init(dir).expect("couldn't make fixture repo");
    let config = r#"[user]
        name = "Anne Other"
        email = "anne@example.com
        "#;
    std::fs::write(dir.to_string() + "/.git/config", config).expect("couldn't write config");
    let sig = repo.signature().expect("couldn't make signature");
    let tree_id = {
        let mut index = repo.index().expect("couldn't get index");
        index.write_tree().expect("couldn't write tree")
    };

    let tree = repo.find_tree(tree_id).expect("couldn't find tree");

    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
        .expect("couldn't commit");

    assert!(
        HashSet::from([Some("master".to_string()), Some("main".to_string())]).contains(&head(dir))
    );
}

#[test]
fn head_of_no_commits_is_none() {
    let dir = "/tmp/my-fixture";
    let _maybe_remove = std::fs::remove_dir_all(dir);
    Repository::init(dir).expect("couldn't make fixture repo");
    assert_eq!(head(dir), None);
}
