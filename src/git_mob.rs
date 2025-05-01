use crate::config;
use crate::io::write_file;
use crate::output::trailers;
use crate::output::MainResult;
use clap::Parser;
use std::fs::File;

#[derive(Parser, Debug, Default)]
pub struct Args {
    #[arg(short, long)]
    pub pick: bool,

    #[arg(short, long)]
    pub message: Option<String>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub initials: Vec<String>,
}

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
    pub template: String,
    pub mob: config::CurrentMobInitials,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn process(team: &config::Team, mob: &config::CurrentMobInitials, args: &Args) -> Output {
    let initials = config::CurrentMobInitials::from_iter(args.initials.iter().cloned());

    if initials.is_empty() {
        output(&args.message, &trailers(team, mob), mob)
    } else {
        output(&args.message, &trailers(team, &initials), &initials)
    }
}

pub fn output(
    message: &Option<String>,
    formatted_trailers: &str,
    mob: &config::CurrentMobInitials,
) -> Output {
    let preamble = match message {
        Some(msg) => msg.to_owned(),
        None => "".to_owned(),
    };
    let msg = format!("{}\n\n{}", preamble, formatted_trailers.to_owned())
        .trim()
        .to_owned();

    Output {
        message: if msg.is_empty() {
            "".to_owned()
        } else {
            format!("{}\n", msg)
        },
        template: if msg.is_empty() {
            "".to_owned()
        } else {
            format!("\n\n{}\n", msg)
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
