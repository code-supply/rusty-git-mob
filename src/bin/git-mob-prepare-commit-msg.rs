use std::io::Read;

use git2::Repository;

use rusty_git_mob::core::*;
use rusty_git_mob::env;
use rusty_git_mob::prepare_commit_message::*;
use rusty_git_mob::writer::*;

fn main() -> MainResult {
    let args = parse_args();
    let env = env::load()?;

    let mut message_file = open_read_write(args.message_path.into())?;
    let mut message = String::new();

    match message_file.read_to_string(&mut message) {
        Ok(_) => {
            let repo = Repository::open(".").unwrap();
            let head = repo.head().unwrap();
            let branch_name = head.shorthand();
            let output = prepare_commit_message(
                &env.coauthors_config.coauthors,
                &env.mob,
                message,
                branch_name,
            );

            write_file(&message_file, &output.message)?;

            Ok(())
        }
        Err(_) => todo!(),
    }
}
