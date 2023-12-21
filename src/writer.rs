use std::fs::File;
use std::io;
use std::io::Seek;
use std::io::Write;

use crate::git_mob::Output;

pub fn write(
    template_file: &File,
    mob_file: &File,
    output: Output,
) -> Result<(), Box<dyn std::error::Error>> {
    write_file(template_file, &output.template)?;

    let mob_json = serde_json::to_string(&output.mob)? + "\n";
    write_file(mob_file, &mob_json)?;

    println!("{}", output.message);

    Ok(())
}

fn write_file(mut file: &File, contents: &str) -> io::Result<()> {
    file.set_len(0)?;
    file.rewind()?;
    file.write_all(contents.as_bytes())
}
