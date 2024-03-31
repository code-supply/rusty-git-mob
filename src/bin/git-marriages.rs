use rusty_git_mob::env;
use rusty_git_mob::git::mob_tally;
use rusty_git_mob::git_marriages::process;
use rusty_git_mob::output::MainResult;

fn main() -> MainResult {
    let env = env::load()?;
    let output = process(env.org, || mob_tally("."));
    println!("{}", output.unwrap().message);
    Ok(())
}
