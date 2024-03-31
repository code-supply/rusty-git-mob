use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json;

use crate::config;
use crate::io::open_read_write;

pub struct Env {
    pub mob_file: File,
    pub mob: config::CurrentMobInitials,
    pub template_file: File,
    pub org: config::Org,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
struct CoauthorsConfig {
    pub teams: config::Org,
}

pub fn load() -> Result<Env, Box<dyn std::error::Error>> {
    let coauthors_path = resolve_path("GIT_MOB_COAUTHORS", ".git-coauthors")?;
    let mob_path = resolve_path("GIT_MOB_LIST", ".git-mob")?;
    let template_path = resolve_path("GIT_MOB_TEMPLATE", ".gitmessage.txt")?;

    let coauthors_file = File::open(coauthors_path).expect("coauthors file error");
    let mob_file = open_read_write(mob_path).expect("mob file error");
    let template_file = open_read_write(template_path).expect("template file error");

    let coauthors_config: CoauthorsConfig =
        serde_json::from_reader(BufReader::new(coauthors_file))?;

    let mob: serde_json::Result<Vec<String>> = serde_json::from_reader(BufReader::new(&mob_file));
    let mob_currently_set = match mob {
        Ok(mob) => config::CurrentMobInitials::from_iter(mob.iter().cloned()),
        Err(_) => config::CurrentMobInitials::default(),
    };

    Ok(Env {
        mob_file,
        mob: mob_currently_set,
        template_file,
        org: coauthors_config.teams,
    })
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

#[cfg(test)]
mod tests;
