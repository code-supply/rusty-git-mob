use rusty_git_mob::git::mob_tally;
use rusty_git_mob::git_marriages::*;

fn main() -> MainResult {
    let output = process(|| mob_tally("."));
    println!("{}", output.message);
    Ok(())
}
