use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

use crate::core::{open_read_write, resolve_path, Coauthors, Mob};

pub struct Env {
    pub mob_file: File,
    pub mob: Mob,
    pub template_file: File,
    pub coauthors: Coauthors,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CoauthorsConfig {
    pub coauthors: Coauthors,
}

pub fn load() -> Result<Env, Box<dyn std::error::Error>> {
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

    Ok(Env {
        mob_file,
        mob: mob_set,
        template_file,
        coauthors: coauthors_config.coauthors,
    })
}
