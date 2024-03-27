use rusty_git_mob::git::mob_tally;
use rusty_git_mob::git_marriages::process;
use rusty_git_mob::git_marriages::MainResult;

fn main() -> MainResult {
    let output = process(|| mob_tally("."));
    println!("{}", output?.message);
    Ok(())
}
