use super::*;
use crate::config::Author;
use crate::config::Team;
use crate::env;
use std::io::Write;

fn with_coauthors<F>(path: &str, config: &[u8], block: F)
where
    F: Fn(std::result::Result<env::Env, EnvError>) + 'static,
{
    let mut file = File::create(path).unwrap();
    file.write_all(config).unwrap();

    std::env::set_var("GIT_MOB_COAUTHORS", path);

    let git_mob_env = env::load();

    block(git_mob_env);

    let _ignore_errors = std::fs::remove_file(path);
}

#[test]
fn friendly_error_when_coauthors_file_nonexistent() {
    std::env::set_var("GIT_MOB_COAUTHORS", "tmp/defo-non-existent-file");
    assert_eq!(env::load().err(), Some(env::EnvError::MissingCoauthorsFile))
}

#[test]
fn friendly_error_when_coauthors_file_not_json() {
    let json = b"{\"oops";

    std::env::set_var("GIT_MOB_TEMPLATE", "README.md");
    std::env::set_var("GIT_MOB_LIST", "tmp/existent-mob-file");

    with_coauthors("tmp/existent-coauthors", json, |git_mob_env| {
        assert_eq!(
            git_mob_env.err(),
            Some(env::EnvError::MalformedCoauthorsFile)
        )
    });
}

#[test]
fn git_mob_file_gets_initialised_when_non_existent() {
    let json = b"{\"teams\": {}}";

    std::env::set_var("GIT_MOB_TEMPLATE", "README.md");

    let _ignore_errors = std::fs::remove_file("tmp/mob-file-init");
    std::env::set_var("GIT_MOB_LIST", "tmp/mob-file-init");

    with_coauthors("tmp/coauthors-init-mob-file-test", json, |git_mob_env| {
        assert_eq!(
            config::CurrentMobInitials::default(),
            git_mob_env.unwrap().mob
        );
    });
}

#[test]
fn authors_can_exclude_alternate_emails() {
    let json = b"{\"teams\": { \"main\": { \"ab\": { \"name\": \"Andrew Bruce\", \"email\": \"me@andrewbruce.net\" } } }}";

    with_coauthors("tmp/empty-alternates", json, |git_mob_env| {
        let expected_org = config::Org::from([(
            "main".to_owned(),
            Team::from([(
                "ab".to_owned(),
                Author::new("Andrew Bruce", "me@andrewbruce.net"),
            )]),
        )]);
        assert_eq!(expected_org, git_mob_env.unwrap().org);
    });
}
