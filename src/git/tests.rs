use super::*;
use crate::core::Author;
use crate::core::Mob;
use git2::Commit;
use std::collections::HashSet;

#[test]
fn can_get_mob_tally_from_multiple_commits() -> Result<()> {
    let dir = "tmp/mob-tally-multiple";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));

    initial_commit(
        &repo,
        "Initial commit
co-AuthoReD-By: Andrew Bruce <me@andrewbruce.net>",
    )?;
    commit(
        &repo,
        "Another commit

Co-authored-By: Maggie Hamilton <margaret@example.com>
Co-authored-By: Mr Potato Head <tatties@example.com>
",
    )?;
    commit(
        &repo,
        "Yet another commit
co-authored-by: Andrew Bruce <me@andrewbruce.net>",
    )?;

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
    let anne_maggie_and_mr_potato_head = Mob::from([
        Author {
            name: "Anne Other".to_owned(),
            email: "anne@example.com".to_owned(),
        },
        Author {
            name: "Maggie Hamilton".to_owned(),
            email: "margaret@example.com".to_owned(),
        },
        Author {
            name: "Mr Potato Head".to_owned(),
            email: "tatties@example.com".to_owned(),
        },
    ]);

    assert_eq!(2, tally.len());
    assert_eq!(Some(&2usize), tally.get(&anne_and_andrew));
    assert_eq!(Some(&1usize), tally.get(&anne_maggie_and_mr_potato_head));
    Ok(())
}

#[test]
fn can_get_mob_tally_from_commit_history_of_one() -> Result<()> {
    let dir = "tmp/mob-tally";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));

    initial_commit(
        &repo,
        "Initial commit
co-AuthoReD-By: Andrew Bruce <me@andrewbruce.net>",
    )?;

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
    assert_eq!(1, *tally.get(&expected_mob).expect("Mob not in result"));

    Ok(())
}

#[test]
fn can_get_mob_from_commit_without_trailers() -> Result<()> {
    let dir = "tmp/authors-no-trailers";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));
    let oid = initial_commit(&repo, "Initial commit")?;

    assert_eq!(
        Ok(Mob::from([Author {
            name: "Anne Other".to_owned(),
            email: "anne@example.com".to_owned()
        }])),
        commit_mob(dir, oid)
    );

    Ok(())
}

#[test]
fn head_of_non_repository_is_none() {
    assert_eq!(head("/tmp"), None);
}

#[test]
fn head_of_one_commit_is_a_short_string() -> Result<()> {
    let dir = "tmp/my-fixture-2";
    let repo = repo(dir, &committer_config("Anne Other", "anne@example.com"));
    initial_commit(&repo, "Initial commit")?;

    assert!(
        HashSet::from([Some("master".to_owned()), Some("main".to_owned())]).contains(&head(dir))
    );

    Ok(())
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

fn initial_commit(repo: &Repository, message: &str) -> Result<Oid> {
    do_commit(repo, message, &[])
}

fn commit(repo: &Repository, message: &str) -> Result<Oid> {
    do_commit(
        repo,
        message,
        &[&repo.find_commit(repo.head()?.target().expect("Couldn't get HEAD target"))?],
    )
}

fn do_commit(repo: &Repository, message: &str, parents: &[&Commit<'_>]) -> Result<Oid> {
    let tree = repo.find_tree({
        let mut index = repo.index()?;
        index.write_tree()?
    })?;

    let sig = repo.signature()?;
    Ok(repo.commit(Some("HEAD"), &sig, &sig, message, &tree, parents)?)
}

fn committer_config(name: &str, email: &str) -> String {
    format!(
        r#"[user]
        name = "{}"
        email = "{}"#,
        name, email
    )
}
