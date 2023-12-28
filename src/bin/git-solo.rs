use rusty_git_mob::core::*;
use rusty_git_mob::env;
use rusty_git_mob::git_mob::*;

fn main() -> MainResult {
    let env = env::load()?;
    let output = output("", &Mob::new());
    write(&env.template_file, &env.mob_file, output)
}
