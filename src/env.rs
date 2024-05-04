use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json;

use crate::config;
use crate::io::open_read_write;

#[derive(Debug)]
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

type Result<T> = std::result::Result<T, EnvError>;

#[derive(Debug, PartialEq)]
pub enum EnvError {
    MalformedCoauthorsFile,
    MissingCoauthorsFile,
}

impl From<serde_json::Error> for EnvError {
    fn from(_value: serde_json::Error) -> Self {
        Self::MalformedCoauthorsFile
    }
}

impl From<std::string::String> for EnvError {
    fn from(_value: std::string::String) -> Self {
        todo!()
    }
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::MissingCoauthorsFile => {
                write!(f, "Please create a coauthors file")
            }
            EnvError::MalformedCoauthorsFile => {
                write!(f, "Your coauthors file is not valid JSON!")
            }
        }
    }
}

impl std::error::Error for EnvError {}

pub fn load() -> Result<Env> {
    let coauthors_path = resolve_path("GIT_MOB_COAUTHORS", ".git-coauthors")?;
    let mob_path = resolve_path("GIT_MOB_LIST", ".git-mob")?;
    let template_path = resolve_path("GIT_MOB_TEMPLATE", ".gitmessage.txt")?;

    let coauthors_file = File::open(coauthors_path).or(Err(EnvError::MissingCoauthorsFile))?;
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

fn resolve_path(env_var_name: &str, filename: &str) -> std::result::Result<PathBuf, String> {
    std::env::var(env_var_name)
        .map(PathBuf::from)
        .or_else(|_e| {
            home::home_dir()
                .map(|path_buf| path_buf.as_path().join(filename))
                .ok_or_else(|| format!("{} not set and couldn't find your home dir!", env_var_name))
        })
}

#[cfg(test)]
mod tests;
