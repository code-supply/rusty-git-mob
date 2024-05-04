use super::*;
use crate::config::Author;
use crate::config::Team;
use crate::env;
use std::io::Write;

fn with_files<F>(coauthors_path: &str, mob_file_path: &str, config: &[u8], block: F)
where
    F: Fn(std::result::Result<env::Env, EnvError>) + 'static,
{
    let mut file = File::create(coauthors_path).unwrap();
    file.write_all(config).unwrap();

    let git_mob_env = env::process(
        Ok(coauthors_path.to_owned()),
        Ok(mob_file_path.to_owned()),
        Ok("README.md".to_owned()),
    );

    block(git_mob_env);

    let _ignore_errors = std::fs::remove_file(coauthors_path);
}

#[test]
fn friendly_error_when_coauthors_file_nonexistent() {
    assert_eq!(
        env::process(
            Ok("tmp/defo-non-existent-file".to_owned()),
            Ok("tmp/existent-mob-file".to_owned()),
            Ok("README.md".to_owned())
        )
        .err(),
        Some(env::EnvError::MissingCoauthorsFile)
    )
}

#[test]
fn friendly_error_when_coauthors_file_not_json() {
    let json = b"{\"oops";

    with_files(
        "tmp/existent-coauthors",
        "tmp/existent-mob-file",
        json,
        |git_mob_env| {
            assert_eq!(
                git_mob_env.err(),
                Some(env::EnvError::MalformedCoauthorsFile)
            )
        },
    );
}

#[test]
fn git_mob_file_gets_initialised_when_non_existent() {
    let json = b"{\"teams\": {}}";

    let _ignore_errors = std::fs::remove_file("tmp/mob-file-init");

    with_files(
        "tmp/coauthors-init-mob-file-test",
        "tmp/mob-file-init",
        json,
        |git_mob_env| {
            assert_eq!(
                config::CurrentMobInitials::default(),
                git_mob_env.unwrap().mob
            );
        },
    );
}

#[test]
fn authors_can_exclude_alternate_emails() {
    let json = b"{\"teams\": { \"main\": { \"ab\": { \"name\": \"Andrew Bruce\", \"email\": \"me@andrewbruce.net\" } } }}";

    with_files(
        "tmp/empty-alternates",
        "tmp/existent-mob-file",
        json,
        |git_mob_env| {
            let expected_org = config::Org::from([(
                "main".to_owned(),
                Team::from([(
                    "ab".to_owned(),
                    Author::new("Andrew Bruce", "me@andrewbruce.net"),
                )]),
            )]);
            assert_eq!(expected_org, git_mob_env.unwrap().org);
        },
    );
}
