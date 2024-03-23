use super::*;
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

    assert_eq!(Mob::default(), git_mob_env.mob);

    let _ignore_errors = std::fs::remove_file("tmp/existent-coauthors");
    let _ignore_errors = std::fs::remove_file("tmp/existent-template");
}
