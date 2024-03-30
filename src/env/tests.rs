use super::*;
use crate::core::Author;
use crate::core::Team;
use crate::env;
use std::io::Write;

fn with_coauthors<F>(path: &str, config: &[u8], block: F)
where
    F: Fn(Env) + 'static,
{
    let mut file = File::create(path).unwrap();
    file.write_all(config).unwrap();

    std::env::set_var("GIT_MOB_COAUTHORS", path);

    let git_mob_env = env::load().unwrap();

    block(git_mob_env);

    let _ignore_errors = std::fs::remove_file(path);
}

#[test]
fn git_mob_file_gets_initialised_when_non_existent() {
    let _ignore_errors = std::fs::remove_file("tmp/non-existent-file");
    let json = b"{\"teams\": {}}";

    std::env::set_var("GIT_MOB_LIST", "tmp/non-existent-file");
    std::env::set_var("GIT_MOB_TEMPLATE", "tmp/existent-template");

    with_coauthors("tmp/existent-coauthors", json, |git_mob_env| {
        assert_eq!(InputMob::default(), git_mob_env.mob);
    });

    let _ignore_errors = std::fs::remove_file("tmp/existent-template");
}

#[test]
fn authors_can_exclude_alternate_emails() {
    let json = b"{\"teams\": { \"main\": { \"ab\": { \"name\": \"Andrew Bruce\", \"email\": \"me@andrewbruce.net\" } } }}";

    with_coauthors("tmp/empty-alternates", json, |git_mob_env| {
        let expected_org = Org::from([(
            "main".to_owned(),
            Team::from([(
                "ab".to_owned(),
                Author::new("Andrew Bruce", "me@andrewbruce.net"),
            )]),
        )]);
        assert_eq!(expected_org, git_mob_env.org);
    });
}
