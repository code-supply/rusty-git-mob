use std::fs::File;
use std::io::BufReader;

use clap::Parser;

mod git_mob;

#[derive(Parser, Debug)]
struct Args {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    initials: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mob_file = File::open("/home/andrew/.git-mob").expect("couldn't read mob list!");
    let mob_reader = BufReader::new(mob_file);
    let mob: Vec<String> = serde_json::from_reader(mob_reader).expect("couldn't parse mob list!");

    let coauthors_file =
        File::open("/home/andrew/.git-coauthors").expect("couldn't read coauthors!");
    let coauthors_reader = BufReader::new(coauthors_file);
    let coauthors: git_mob::Coauthors =
        serde_json::from_reader(coauthors_reader).expect("couldn't parse coauthors!");

    println!("starting mob: {:?}", mob);

    let effect = git_mob::process(&coauthors, mob, args.initials);

    println!("effect would be: {:?}", effect);
}
