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
    let ts = &trailers(coauthors, mob);

    if ts.is_empty() {
        PrepareCommitMessageOutput { message }
    } else {
        PrepareCommitMessageOutput {
            message: format!("{}\n\n{}", message, ts),
        }
    }
}

#[cfg(test)]
mod tests;
