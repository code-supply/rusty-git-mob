use std::fs::File;
use std::io::BufReader;

use git_mob::core::*;
use git_mob::git_mob_cmd::*;
use git_mob::picker;

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
            move |output: GitMobOutput| write(&template_file, &mob_file, output),
        );
        Ok(())
    } else {
        let output = process(&coauthors_config.coauthors, &mob_set, &args);
        write(&template_file, &mob_file, output)
    }
}
