#![allow(unused)]
use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    ///The patern to look for
    pattern : String,
    ///The path to the file to read
    path : std::path::PathBuf,
}



fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read the file `{}`", args.path.display()))?;
    Grep_tool::find_matching_strings(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]

fn validate() {
    let mut result = Vec::new();
    Grep_tool::find_matching_strings("Ana are mere", "Ana", &mut result);
    assert_eq!(result, b"Ana are mere\n");
}

