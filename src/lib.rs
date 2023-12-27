use clap::Parser;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::{env, io};

pub type MainResult = Result<(), Box<dyn std::error::Error>>;
pub type Mob = BTreeSet<String>;
pub type Coauthors = BTreeMap<String, Coauthor>;

#[derive(Parser, Debug, Default)]
pub struct GitMobArgs {
    #[arg(short, long)]
    solo: bool,

    #[arg(short, long)]
    pub pick: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    initials: Vec<String>,
}

#[derive(Parser, Debug, Default)]
pub struct PrepareCommitMessageArgs {
    pub message_path: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CoauthorsConfig {
    pub coauthors: Coauthors,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Coauthor {
    pub name: String,
    email: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct GitMobOutput {
    pub message: String,
    pub template: String,
    pub mob: Mob,
}

#[derive(Debug, PartialEq, Default)]
pub struct PrepareCommitMessageOutput {
    pub message: String,
}

pub fn parse_args() -> GitMobArgs {
    GitMobArgs::parse()
}

pub fn parse_prepare_commit_message_args() -> PrepareCommitMessageArgs {
    PrepareCommitMessageArgs::parse()
}

pub fn prepare_commit_message(
    coauthors: &Coauthors,
    mob: &Mob,
    message: String,
) -> PrepareCommitMessageOutput {
    let ts = &trailers(coauthors, mob);

    if ts.is_empty() {
        PrepareCommitMessageOutput { message }
    } else {
        PrepareCommitMessageOutput {
            message: format!("{}\n\n{}", message, ts),
        }
    }
}

pub fn process(coauthors: &Coauthors, mob: &Mob, args: &GitMobArgs) -> GitMobOutput {
    let initials = Mob::from_iter(args.initials.iter().cloned());

    if args.solo {
        output("", &Mob::new())
    } else if initials.is_empty() {
        output(&trailers(coauthors, mob), mob)
    } else {
        output(&trailers(coauthors, &initials), &initials)
    }
}

pub fn output(formatted_trailers: &str, mob: &Mob) -> GitMobOutput {
    GitMobOutput {
        message: formatted_trailers.to_string(),
        template: if formatted_trailers.is_empty() {
            "".to_string()
        } else {
            format!("\n\n{}", formatted_trailers)
        },
        mob: mob.clone(),
    }
}

pub fn trailers(coauthors: &Coauthors, initials: &Mob) -> String {
    initials
        .iter()
        .fold(String::new(), |acc, initial| match coauthors.get(initial) {
            Some(coauthor) => {
                format!(
                    "{}Co-authored-by: {} <{}>\n",
                    acc, coauthor.name, coauthor.email
                )
            }
            None => acc,
        })
}

pub fn open_read_write(path: PathBuf) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}

pub fn resolve_path(env_var_name: &str, filename: &str) -> Result<PathBuf, String> {
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
mod tests {
    use super::*;

    #[test]
    fn prepare_commit_message_with_empty_coauthors_produces_empty_message() {
        assert_eq!(
            prepare_commit_message(&Coauthors::default(), &Mob::default(), "".to_string()),
            PrepareCommitMessageOutput::default()
        );
    }

    #[test]
    fn prepare_commit_message_with_empty_coauthors_and_only_comments_has_no_leading_whitespace() {
        assert_eq!(
            prepare_commit_message(
                &Coauthors::default(),
                &Mob::default(),
                "# original comment".to_string()
            ),
            PrepareCommitMessageOutput {
                message: "# original comment".to_string()
            }
        );
    }

    #[test]
    fn prepare_commit_message_adds_coauthors_to_message_without_comments() {
        assert_eq!(
            prepare_commit_message(
                &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
                &Mob::from(["ab".to_string(), "fb".to_string()]),
                "Hello, World!".to_string()
            ),
            PrepareCommitMessageOutput {
                message: r#"Hello, World!

Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>
"#
                .to_string()
            }
        )
    }

    #[test]
    fn empty_input_returns_empty_output() {
        assert_eq!(
            process(
                &coauthors(&Mob::new()),
                &Mob::new(),
                &GitMobArgs {
                    initials: vec![],
                    ..Default::default()
                }
            ),
            GitMobOutput::default()
        );
    }

    #[test]
    fn forming_a_mob_outputs_the_mob_and_template() {
        assert_eq!(
            process(
                &coauthors(&Mob::from(["ab".to_string()])),
                &Mob::new(),
                &GitMobArgs {
                    initials: vec!["ab".to_string()],
                    ..Default::default()
                }
            ),
            GitMobOutput {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
                template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
                mob: Mob::from(["ab".to_string()]),
            }
        );
    }

    #[test]
    fn can_add_many_mobsters() {
        assert_eq!(
            process(
                &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
                &Mob::new(),
                &GitMobArgs {
                    initials: vec!["ab".to_string(), "fb".to_string()],
                    ..Default::default()
                }
            ),
            GitMobOutput {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_string(),
                template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_string(),
                mob: Mob::from(["ab".to_string(), "fb".to_string()]),
            }
        );
    }

    #[test]
    fn calling_without_initials_outputs_current_mob() {
        assert_eq!(
            process(
                &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
                &Mob::from(["ab".to_string(), "fb".to_string()]),
                &GitMobArgs {
                    initials: vec![],
                    ..Default::default()
                },
            ),
            GitMobOutput {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_string(),
                template: "\n\nCo-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_string(),
                mob: Mob::from(["ab".to_string(), "fb".to_string()]),
            }
        )
    }

    #[test]
    fn soloing_shows_no_output_and_wipes_mob_and_template() {
        assert_eq!(
            process(
                &coauthors(&Mob::from(["ab".to_string(), "fb".to_string()])),
                &Mob::from(["ab".to_string(), "fb".to_string()]),
                &GitMobArgs {
                    initials: vec!["ab".to_string()],
                    solo: true,
                    pick: false
                },
            ),
            GitMobOutput {
                message: "".to_string(),
                template: "".to_string(),
                mob: Mob::new(),
            }
        )
    }

    #[test]
    fn coauthors_are_sorted_by_initials() {
        let a = coauthors(&Mob::from(["ab".to_string(), "fb".to_string()]));
        let expected: Vec<_> = a.iter().collect();
        let b = coauthors(&Mob::from(["fb".to_string(), "ab".to_string()]));
        let actual: Vec<_> = b.iter().collect();
        assert_eq!(actual, expected)
    }

    #[test]
    fn mob_is_sorted() {
        let a = Mob::from(["ab".to_string(), "fb".to_string()]);
        let expected: Vec<_> = a.iter().collect();
        let b = Mob::from(["fb".to_string(), "ab".to_string()]);
        let actual: Vec<_> = b.iter().collect();
        assert_eq!(actual, expected)
    }

    fn coauthors(initials: &Mob) -> Coauthors {
        Coauthors::from([
            (
                "ab".to_string(),
                Coauthor {
                    name: "Andrew Bruce".to_string(),
                    email: "me@andrewbruce.net".to_string(),
                },
            ),
            (
                "fb".to_string(),
                Coauthor {
                    name: "Fred Brookes".to_string(),
                    email: "fred@example.com".to_string(),
                },
            ),
        ])
        .into_iter()
        .filter(|(k, _v)| initials.contains(k))
        .collect()
    }
}
