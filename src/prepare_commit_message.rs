use clap::Parser;

use crate::core::*;

#[derive(Parser, Debug, Default)]
pub struct Args {
    pub message_path: String,
}

pub fn prepare_commit_message(
    coauthors: &Coauthors,
    mob: &Mob,
    message: String,
) -> PrepareCommitMessageOutput {
    let trails = &trailers(coauthors, mob);

    if trails.is_empty() {
        PrepareCommitMessageOutput { message }
    } else {
        let parts: Vec<&str> = message.splitn(2, "\n#").collect();

        match parts[..] {
            [before, after] => PrepareCommitMessageOutput {
                message: format!("{}\n{}\n#{}", before, trails, after),
            },
            _ => PrepareCommitMessageOutput {
                message: format!("{}\n\n{}", message, trails),
            },
        }
    }
}

#[cfg(test)]
mod tests;
