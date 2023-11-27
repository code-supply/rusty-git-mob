use crate::git_mob::CoauthorsConfig;
use clap::Parser;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

mod git_mob;

#[derive(Parser, Debug)]
struct Args {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    initials: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let git_mob_list_path = resolve_path("GIT_MOB_LIST", ".git-mob")?;
    let git_coauthors_path = resolve_path("GIT_MOB_COAUTHORS", ".git-coauthors")?;
    let template_path = resolve_path("GIT_MOB_TEMPLATE", ".gitmessage.txt")?;

    let mob_file = open_read_write(git_mob_list_path)?;
    let template_file = open_read_write(template_path)?;

    let mob_reader = BufReader::new(&mob_file);
    let mob: Vec<String> = serde_json::from_reader(mob_reader)?;

    let coauthors_file = File::open(git_coauthors_path)?;
    let coauthors_reader = BufReader::new(coauthors_file);
    let coauthors_config: CoauthorsConfig = serde_json::from_reader(coauthors_reader)?;

    let output = git_mob::process(&coauthors_config.coauthors, mob, &args.initials);

    write(template_file, &output.template)?;

    let mob_json = serde_json::to_string(&output.mob)? + "\n";
    write(mob_file, &mob_json)?;

    println!("{}", output.message);

    Ok(())
}

fn write(mut file: File, contents: &str) -> io::Result<()> {
    file.set_len(0)?;
    file.rewind()?;
    file.write_all(contents.as_bytes())
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
