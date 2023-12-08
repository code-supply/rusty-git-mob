use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Parser, Debug, Default)]
pub struct Args {
    #[arg(short, long)]
    solo: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    initials: Vec<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[derive(Debug, PartialEq)]
pub struct Output {
    pub message: String,
    pub template: String,
    pub mob: Vec<String>,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            template: "".to_string(),
            mob: vec![],
        }
    }
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
    name: String,
    email: String,
}

pub(crate) fn process(coauthors: &Coauthors, mob: &[String], args: &Args) -> Output {
    let initials = &args.initials;

    if args.solo {
        output("", &[])
    } else if initials.is_empty() {
        output(&trailers(coauthors, mob), mob)
    } else {
        output(&trailers(coauthors, initials), initials)
    }
}

fn output(formatted_trailers: &str, mob: &[String]) -> Output {
    Output {
        message: formatted_trailers.to_string(),
        template: formatted_trailers.to_string(),
        mob: mob.to_vec(),
    }
}

fn trailers(coauthors: &Coauthors, initials: &[String]) -> String {
    initials.iter().fold(String::new(), |acc, initial| {
        if let Some(coauthor) = coauthors.get(initial) {
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
                &Coauthors::default(),
                &[],
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
        let coauthors = Coauthors::from([(
            "ab".to_string(),
            Coauthor {
                name: "Andrew Bruce".to_string(),
                email: "me@andrewbruce.net".to_string(),
            },
        )]);

        assert_eq!(
            process(
                &coauthors,
                &[],
                &Args {
                    initials: vec!["ab".to_string()],
                    ..Default::default()
                }
            ),
            Output {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
                template: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
                mob: vec!["ab".to_string()],
            }
        );
    }

    #[test]
    fn can_add_many_mobsters() {
        let coauthors = Coauthors::from([
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
        ]);

        assert_eq!(
            process(
                &coauthors,
                &[],
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
                mob: vec!["ab".to_string(), "fb".to_string()],
            }
        );
    }

    #[test]
    fn calling_without_initials_outputs_current_mob() {
        let coauthors = Coauthors::from([
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
        ]);

        assert_eq!(
            process(
                &coauthors,
                &["ab".to_string(), "fb".to_string()],
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
                mob: vec!["ab".to_string(), "fb".to_string()],
            }
        )
    }

    #[test]
    fn soloing_shows_no_output_and_wipes_mob_and_template() {
        let coauthors = Coauthors::from([
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
        ]);

        assert_eq!(
            process(
                &coauthors,
                &["ab".to_string(), "fb".to_string()],
                &Args {
                    initials: vec!["ab".to_string()],
                    solo: true
                },
            ),
            Output {
                message: "".to_string(),
                template: "".to_string(),
                mob: vec![],
            }
        )
    }
}
