use rusty_git_mob::env;
use rusty_git_mob::git;
use rusty_git_mob::io::open_read_write;
use rusty_git_mob::io::write_file;
use rusty_git_mob::output::MainResult;
use rusty_git_mob::prepare_commit_message::*;
use std::io::Read;

fn main() -> MainResult {
    let args = parse_args();
    let env = env::load()?;

    let mut message_file = open_read_write(args.message_path.into())?;
    let mut message = String::new();
    message_file.read_to_string(&mut message)?;

    let output = prepare_commit_message(&env.org, &env.mob, message, git::head(".").as_deref());

    Ok(write_file(&message_file, &output.message)?)
}
