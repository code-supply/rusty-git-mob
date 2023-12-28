use std::io::Read;

use git2::Repository;

use rusty_git_mob::core::*;
use rusty_git_mob::env;
use rusty_git_mob::prepare_commit_message::*;

fn main() -> MainResult {
    let args = parse_args();
    let env = env::load()?;

    let mut message_file = open_read_write(args.message_path.into())?;
    let mut message = String::new();
    message_file.read_to_string(&mut message)?;

    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let output = prepare_commit_message(&env.coauthors, &env.mob, message, head.shorthand());

    Ok(write_file(&message_file, &output.message)?)
}
