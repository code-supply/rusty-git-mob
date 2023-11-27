use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Output {
    message: String,
    template: String,
    mob: Vec<String>,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            message: "".to_owned(),
            template: "".to_owned(),
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

pub(crate) fn process(coauthors: &Coauthors, _mob: Vec<String>, initials: &[String]) -> Output {
    let trailers = trailers(coauthors, initials);

    Output {
        message: trailers.to_string(),
        template: trailers,
        mob: initials.to_vec(),
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
    fn test_empty_input_returns_empty_output() {
        assert_eq!(
            process(&Coauthors::default(), vec![], &[]),
            Output::default()
        );
    }

    #[test]
    fn test_add_mobster_to_empty_mob() {
        let coauthors = Coauthors::from([(
            "ab".to_string(),
            Coauthor {
                name: "Andrew Bruce".to_string(),
                email: "me@andrewbruce.net".to_string(),
            },
        )]);

        assert_eq!(
            process(&coauthors, vec![], &["ab".to_string()]),
            Output {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
                template: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>\n".to_owned(),
                mob: vec!["ab".to_string()],
            }
        );
    }

    #[test]
    fn test_add_many_mobsters_to_empty_mob() {
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
            process(&coauthors, vec![], &["ab".to_string(), "fb".to_string()]),
            Output {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_owned(),
                template: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>
Co-authored-by: Fred Brookes <fred@example.com>\n"
                    .to_owned(),
                mob: vec!["ab".to_string(), "fb".to_string()],
            }
        );
    }
}
