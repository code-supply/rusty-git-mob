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
pub struct Coauthors {
    coauthors: HashMap<String, Coauthor>,
}

impl Default for Coauthors {
    fn default() -> Self {
        Self {
            coauthors: HashMap::from([]),
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
struct Coauthor {
    name: String,
    email: String,
}

pub(crate) fn process(coauthors: &Coauthors, _mob: Vec<String>, initials: Vec<String>) -> Output {
    if !initials.is_empty() {
        let first_initial = initials.first().expect("BAD STUFF");
        let coauthor = coauthors.coauthors.get(first_initial).expect("BAD STUFF");
        let formatted = format!("Co-authored-by: {} <{}>", coauthor.name, coauthor.email);

        Output {
            message: formatted.to_string(),
            template: formatted,
            mob: initials.clone(),
        }
    } else {
        Output::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input_returns_empty_output() {
        assert_eq!(
            process(&Coauthors::default(), vec![], vec![]),
            Output::default()
        );
    }

    #[test]
    fn test_add_mobster_to_empty_mob() {
        let coauthors = Coauthors {
            coauthors: HashMap::from([(
                "ab".to_string(),
                Coauthor {
                    name: "Andrew Bruce".to_string(),
                    email: "me@andrewbruce.net".to_string(),
                },
            )]),
        };

        assert_eq!(
            process(&coauthors, vec![], vec!["ab".to_string()]),
            Output {
                message: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>".to_owned(),
                template: "Co-authored-by: Andrew Bruce <me@andrewbruce.net>".to_owned(),
                mob: vec!["ab".to_string()],
            }
        );
    }
}
