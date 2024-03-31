use rusty_git_mob::config::*;
use rusty_git_mob::env;
use rusty_git_mob::git_mob::*;
use rusty_git_mob::output::MainResult;

fn main() -> MainResult {
    let env = env::load()?;
    let output = output("", &CurrentMobInitials::new());
    write(&env.template_file, &env.mob_file, output)
}
