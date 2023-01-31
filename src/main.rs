use minigrep::{GrepConfig, ConvertibleToGrepResult};
use regex::Regex;
use std::fs::File;

fn main() -> minigrep::Result<()> {
    let config = GrepConfig::from_cli(std::env::args())?;
    let file = File::open(config.path).to_grep_result()?;
    let pattern = Regex::new(&config.pattern).to_grep_result()?;
    minigrep::grep(file, pattern)
}
