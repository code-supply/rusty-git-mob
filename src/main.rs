use crate::git_mob::CoauthorsConfig;
use crate::git_mob::MainResult;
use crate::git_mob::Mob;
use crate::git_mob::Output;

use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;

mod git_mob;
mod picker;
mod writer;

fn main() -> MainResult {
    let args = git_mob::parse_args();

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
            move |output: Output| writer::write(&template_file, &mob_file, output),
        );
        Ok(())
    } else {
        let output = git_mob::process(&coauthors_config.coauthors, &mob_set, &args);
        writer::write(&template_file, &mob_file, output)
    }
}

fn open_read_write(path: PathBuf) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}

fn resolve_path(env_var_name: &str, filename: &str) -> Result<PathBuf, String> {
    match env::var(env_var_name) {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_e) => match home::home_dir() {
            Some(path_buf) => Ok(path_buf.as_path().join(filename)),
            None => Err(format!(
                "{} not set and couldn't find your home dir!",
                env_var_name
            )),
        },
    }
}
