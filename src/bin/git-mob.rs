use rusty_git_mob::core::whole_org_as_team;
use rusty_git_mob::core::MainResult;
use rusty_git_mob::env;
use rusty_git_mob::git_mob::Output;
use rusty_git_mob::git_mob::*;
use rusty_git_mob::picker;

fn main() -> MainResult {
    let args = parse_args();
    let env = env::load()?;

    if args.pick {
        picker::run(env.org, &env.mob, move |output: Output| {
            write(&env.template_file, &env.mob_file, output)
        });
        Ok(())
    } else {
        write(
            &env.template_file,
            &env.mob_file,
            process(&whole_org_as_team(&env.org), &env.mob, &args),
        )
    }
}
