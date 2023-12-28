use clap::Parser;

use crate::core::*;

pub type MainResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Parser, Debug, Default)]
pub struct Args {
    pub message_path: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn prepare_commit_message(
    coauthors: &Coauthors,
    mob: &Mob,
    message: String,
    branch_name: Option<&str>,
) -> PrepareCommitMessageOutput {
    PrepareCommitMessageOutput {
        message: convert_message(&trailers(coauthors, mob), message, branch_name),
    }
}

fn convert_message(
    configured_trailers: &String,
    message: String,
    branch_name: Option<&str>,
) -> String {
    if branch_name.is_none() || configured_trailers.is_empty() || has_coauthors(&message) {
        message
    } else if is_only_comments(&message) {
        format!("\n{}\n{}", configured_trailers, message)
    } else {
        let parts: Vec<&str> = message.splitn(2, "\n#").collect();

        match parts[..] {
            [before, after] => format!("{}\n{}\n#{}", before, configured_trailers, after),
            _ => format!("{}\n\n{}", message, configured_trailers),
        }
    }
}

fn has_coauthors(message: &str) -> bool {
    message
        .lines()
        .any(|l| l.to_lowercase().starts_with("co-authored-by:"))
}

fn is_only_comments(message: &str) -> bool {
    message.lines().all(|l| l.starts_with('#'))
}

#[cfg(test)]
mod tests;
