use rusty_git_mob::env;
use rusty_git_mob::git_mob::Output;
use rusty_git_mob::git_mob::*;
use rusty_git_mob::picker;

fn main() -> MainResult {
    let args = parse_args();
    let env = env::load()?;

    if args.pick {
        picker::run(env.coauthors, &env.mob, move |output: Output| {
            write(&env.template_file, &env.mob_file, output)
        });
        Ok(())
    } else {
        let output = process(&env.coauthors, &env.mob, &args);
        write(&env.template_file, &env.mob_file, output)
    }
}
