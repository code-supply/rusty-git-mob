use clap::Parser;
use rusty_git_mob::config::*;
use rusty_git_mob::env;
use rusty_git_mob::git_mob::*;
use rusty_git_mob::output::MainResult;

#[derive(Parser, Debug, Default)]
struct Args {
    #[arg(short, long)]
    pub message: Option<String>,
}

fn main() -> MainResult {
    let args = Args::parse();
    let env = env::load()?;

    let msg = match args.message {
        Some(msg) => format!("\n\n{}", msg),
        None => "".to_owned(),
    };

    let output = output(&None, &msg, &CurrentMobInitials::new());

    write(&env.template_file, &env.mob_file, output)
}
