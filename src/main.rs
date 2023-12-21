use crate::git_mob::CoauthorsConfig;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;

use self::git_mob::Output;

mod git_mob;
mod pick_view;
mod writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mob_set: HashSet<String> = HashSet::from_iter(mob.iter().cloned());

    if args.pick {
        pick_view::render(
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

fn resolve_path(env_var_name: &str, filename: &str) -> Result<PathBuf, &'static str> {
    match env::var(env_var_name) {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_e) => match home::home_dir() {
            Some(path_buf) => Ok(path_buf.as_path().join(filename)),
            None => Err("GIT_MOB_LIST not set and couldn't find your home dir!"),
        },
    }
}
