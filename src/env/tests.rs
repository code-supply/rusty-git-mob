use super::*;
use crate::env;

#[test]
fn git_mob_file_gets_initialised_when_non_existent() {
    let _ignore_errors = std::fs::remove_file("/tmp/non-existent-file");
    std::env::set_var("GIT_MOB_LIST", "/tmp/non-existent-file");

    let git_mob_env = env::load().unwrap();

    assert_eq!(Mob::default(), git_mob_env.mob);
}
