use git_mob::open_read_write;
use git_mob::parse_args;
use git_mob::process;
use git_mob::resolve_path;
use git_mob::CoauthorsConfig;
use git_mob::GitMobOutput;
use git_mob::MainResult;
use git_mob::Mob;

use std::fs::File;
use std::io::BufReader;

mod picker;
mod writer;

fn main() -> MainResult {
    let args = parse_args();

    let coauthors_path = resolve_path("GIT_MOB_COAUTHORS", ".git-coauthors")?;
    let mob_path = resolve_path("GIT_MOB_LIST", ".git-mob")?;
    let template_path = resolve_path("GIT_MOB_TEMPLATE", ".gitmessage.txt")?;

    let coauthors_file = File::open(coauthors_path)?;
    let mob_file = open_read_write(mob_path)?;
    let template_file = open_read_write(template_path)?;

    let coauthors_config: CoauthorsConfig =
        serde_json::from_reader(BufReader::new(coauthors_file))?;
    let mob: Vec<String> = serde_json::from_reader(BufReader::new(&mob_file))?;

    let mob_set: Mob = Mob::from_iter(mob.iter().cloned());

    if args.pick {
        picker::run(
            coauthors_config.coauthors,
            &mob_set,
            move |output: GitMobOutput| writer::write(&template_file, &mob_file, output),
        );
        Ok(())
    } else {
        let output = process(&coauthors_config.coauthors, &mob_set, &args);
        writer::write(&template_file, &mob_file, output)
    }
}
