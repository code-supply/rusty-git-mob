use serde::Deserialize;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Clone, Default, Deserialize, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct Author {
    pub name: String,
    email: String,
    alternate_emails: Option<BTreeSet<String>>,
}

impl Author {
    pub fn new(name: &str, email: &str) -> Author {
        Author {
            name: name.to_owned(),
            email: email.to_owned(),
            ..Default::default()
        }
    }

    pub fn new_with_alternates(
        name: &str,
        email: &str,
        alternate_emails: BTreeSet<String>,
    ) -> Author {
        Author {
            name: name.to_owned(),
            email: email.to_owned(),
            alternate_emails: Some(alternate_emails),
        }
    }

    pub fn use_configured(&self, configured_authors: &BTreeSet<Author>) -> Author {
        configured_authors
            .iter()
            .find(|candidate| candidate.is_configured_equivalent_of(self))
            .unwrap_or(self)
            .to_owned()
    }

    fn is_configured_equivalent_of(&self, found_author: &Author) -> bool {
        self.email == found_author.email
            || self
                .alternate_emails
                .to_owned()
                .is_some_and(|e| e.contains(&found_author.email))
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}
