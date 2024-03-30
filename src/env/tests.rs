use super::*;
use crate::core::Author;
use crate::core::Team;
use crate::env;
use std::io::Write;

#[test]
fn git_mob_file_gets_initialised_when_non_existent() {
    let _ignore_errors = std::fs::remove_file("tmp/non-existent-file");

    std::env::set_var("GIT_MOB_LIST", "tmp/non-existent-file");

    let mut file = File::create("tmp/existent-coauthors").unwrap();
    file.write_all(b"{\"teams\": {}}").unwrap();

    std::env::set_var("GIT_MOB_COAUTHORS", "tmp/existent-coauthors");
    std::env::set_var("GIT_MOB_TEMPLATE", "tmp/existent-template");

    let git_mob_env = env::load().unwrap();

    assert_eq!(InputMob::default(), git_mob_env.mob);

    let _ignore_errors = std::fs::remove_file("tmp/existent-coauthors");
    let _ignore_errors = std::fs::remove_file("tmp/existent-template");
}

#[test]
fn authors_can_exclude_alternate_emails() {
    let filename = "tmp/empty-alternates";
    let mut file = File::create(filename).unwrap();
    file.write_all(b"{\"teams\": { \"main\": { \"ab\": { \"name\": \"Andrew Bruce\", \"email\": \"me@andrewbruce.net\" } } }}")
        .unwrap();

    std::env::set_var("GIT_MOB_COAUTHORS", filename);

    let git_mob_env = env::load().unwrap();

    let expected_org = Org::from([(
        "main".to_owned(),
        Team::from([(
            "ab".to_owned(),
            Author::new("Andrew Bruce", "me@andrewbruce.net"),
        )]),
    )]);
    assert_eq!(expected_org, git_mob_env.org);

    let _ignore_errors = std::fs::remove_file(filename);
}
