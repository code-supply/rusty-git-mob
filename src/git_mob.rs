use std::fs::File;

use clap::Parser;

use crate::core::*;

pub type MainResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Parser, Debug, Default)]
pub struct Args {
    #[arg(short, long)]
    pub pick: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub initials: Vec<String>,
}

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
    pub template: String,
    pub mob: Mob,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn process(coauthors: &Coauthors, mob: &Mob, args: &Args) -> Output {
    let initials = Mob::from_iter(args.initials.iter().cloned());

    if initials.is_empty() {
        output(&trailers(coauthors, mob), mob)
    } else {
        output(&trailers(coauthors, &initials), &initials)
    }
}

pub fn output(formatted_trailers: &str, mob: &Mob) -> Output {
    Output {
        message: formatted_trailers.to_owned(),
        template: if formatted_trailers.is_empty() {
            "".to_owned()
        } else {
            format!("\n\n{}", formatted_trailers)
        },
        mob: mob.clone(),
    }
}

pub fn write(template_file: &File, mob_file: &File, output: Output) -> MainResult {
    write_file(template_file, &output.template)?;

    let mob_json = serde_json::to_string(&output.mob)? + "\n";
    write_file(mob_file, &mob_json)?;

    println!("{}", output.message);

    Ok(())
}

#[cfg(test)]
mod tests;
