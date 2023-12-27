use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use git_mob::open_read_write;
use git_mob::resolve_path;
use git_mob::CoauthorsConfig;
use git_mob::MainResult;
use git_mob::Mob;
use git_mob::PrepareCommitMessageOutput;

fn main() -> MainResult {
    let args = git_mob::parse_prepare_commit_message_args();

    let coauthors_path = resolve_path("GIT_MOB_COAUTHORS", ".git-coauthors")?;
    let mob_path = resolve_path("GIT_MOB_LIST", ".git-mob")?;

    let coauthors_file = File::open(coauthors_path)?;
    let mut message_file = open_read_write(args.message_path.into())?;
    let mob_file = open_read_write(mob_path)?;

    let mut message = String::new();

    match message_file.read_to_string(&mut message) {
        Ok(_size) => {
            let coauthors_config: CoauthorsConfig =
                serde_json::from_reader(BufReader::new(coauthors_file))?;
            let mob: Vec<String> = serde_json::from_reader(BufReader::new(&mob_file))?;

            let mob_set: Mob = Mob::from_iter(mob.iter().cloned());

            let PrepareCommitMessageOutput { message: _ } =
                git_mob::prepare_commit_message(&coauthors_config.coauthors, &mob_set, message);
            Ok(())
        }
        Err(_) => todo!(),
    }
}
