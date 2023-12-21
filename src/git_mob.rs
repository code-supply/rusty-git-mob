use clap::Parser;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Parser, Debug, Default)]
pub struct Args {
    #[arg(short, long)]
    solo: bool,

    #[arg(short, long)]
    pub pick: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    initials: Vec<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[derive(Debug, PartialEq, Default)]
pub struct Output {
    pub message: String,
    pub template: String,
    pub mob: HashSet<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CoauthorsConfig {
    pub coauthors: Coauthors,
}

impl Default for CoauthorsConfig {
    fn default() -> Self {
        Self {
            coauthors: HashMap::from([]),
        }
    }
}

pub type Coauthors = HashMap<String, Coauthor>;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Coauthor {
    pub name: String,
    email: String,
}

pub fn process(coauthors: &Coauthors, mob: &HashSet<String>, args: &Args) -> Output {
    let initials = HashSet::from_iter(args.initials.iter().cloned());

    if args.solo {
        output("", &HashSet::new())
    } else if initials.is_empty() {
        output(&trailers(coauthors, mob), mob)
    } else {
        output(&trailers(coauthors, &initials), &initials)
    }
}

pub fn output(formatted_trailers: &str, mob: &HashSet<String>) -> Output {
    Output {
        message: formatted_trailers.to_string(),
        template: formatted_trailers.to_string(),
        mob: mob.clone(),
    }
}

pub fn trailers(coauthors: &Coauthors, initials: &HashSet<String>) -> String {
    let mut sorted = Vec::from_iter(initials);
    sorted.sort();
    sorted.iter().fold(String::new(), |acc, initial| {
        if let Some(coauthor) = coauthors.get(initial.to_owned()) {
            format!(
                "{}Co-authored-by: {} <{}>\n",
                acc, coauthor.name, coauthor.email
            )
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_returns_empty_output() {
        assert_eq!(
            process(
                &coauthors(&HashSet::new()),
                &HashSet::new(),
                &Args {
                    initials: vec![],
                    ..Default::default()
                }
            ),
            Output::default()
        );
    }

    #[test]
    fn forming_a_mob_outputs_the_mob() {
        assert_eq!(
            process(
                &coauthors(&HashSet::from(["ab".to_string()])),
                &HashSet::new(),
                &Args {
                    initials: vec!["ab".to_string()],
                    ..Default::default()
                }
            ),
            Output {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
                template: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
                mob: HashSet::from(["ab".to_string()]),
            }
        );
    }

    #[test]
    fn can_add_many_mobsters() {
        assert_eq!(
            process(
                &coauthors(&HashSet::from(["ab".to_string(), "fb".to_string()])),
                &HashSet::new(),
                &Args {
                    initials: vec!["ab".to_string(), "fb".to_string()],
                    ..Default::default()
                }
            ),
            Output {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_string(),
                template: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_string(),
                mob: HashSet::from(["ab".to_string(), "fb".to_string()]),
            }
        );
    }

    #[test]
    fn calling_without_initials_outputs_current_mob() {
        assert_eq!(
            process(
                &coauthors(&HashSet::from(["ab".to_string(), "fb".to_string()])),
                &HashSet::from(["ab".to_string(), "fb".to_string()]),
                &Args {
                    initials: vec![],
                    ..Default::default()
                },
            ),
            Output {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_string(),
                template: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_string(),
                mob: HashSet::from(["ab".to_string(), "fb".to_string()]),
            }
        )
    }

    #[test]
    fn soloing_shows_no_output_and_wipes_mob_and_template() {
        assert_eq!(
            process(
                &coauthors(&HashSet::from(["ab".to_string(), "fb".to_string()])),
                &HashSet::from(["ab".to_string(), "fb".to_string()]),
                &Args {
                    initials: vec!["ab".to_string()],
                    solo: true,
                    pick: false
                },
            ),
            Output {
                message: "".to_string(),
                template: "".to_string(),
                mob: HashSet::new(),
            }
        )
    }

    fn coauthors(initials: &HashSet<String>) -> Coauthors {
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
