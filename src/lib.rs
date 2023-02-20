use colored::{self, Colorize};
use regex::Regex;
use std::fmt::{Display, Write};
use std::fs::File;

mod regex_slice_iterator;
mod buffer_lines_iterator;

use regex_slice_iterator::RegexSliceIterator;
use buffer_lines_iterator::BufferLinesIterator;

#[derive(Debug)]
pub struct GrepConfig {
    pub pattern: String,
    pub path: String,
}

#[derive(Debug)]
pub struct FileGrepError(String);
pub type Result<T> = std::result::Result<T, FileGrepError>;

impl GrepConfig {
    fn expect_cli_arg<T: Iterator<Item = String>>(args: &mut T) -> Result<String> {
        args.next().ok_or(FileGrepError(
            "Not enough command line paramters".to_string(),
        ))
    }

    pub fn from_cli<T: Iterator<Item = String>>(mut args: T) -> Result<GrepConfig> {
        Self::expect_cli_arg(&mut args)?;
        Ok(GrepConfig {
            pattern: Self::expect_cli_arg(&mut args)?,
            path: Self::expect_cli_arg(&mut args)?,
        })
    }
}

pub fn grep(file: File, pattern: Regex) -> Result<()> {
    let mut line_index:u32 = 0;
    for line in BufferLinesIterator::new(file) {
        let line = line.trim_end_matches('\n');

        let mut last_index = None;
        let mut line_index_str = String::new();
        for rmatch in RegexSliceIterator::new(&pattern, line) {
            if let Some(index) = last_index {
                print!("{}", &line[index..rmatch.start()]);
            } else {
                write!(&mut line_index_str, "{}", line_index).expect("Failed to append string");
                print!("{}: {}", line_index_str.cyan(), &line[0..rmatch.start()]);
            }
            last_index = Some(rmatch.end());
            print!("{}", rmatch.as_str().green());
        }

        if let Some(idx) = last_index {
            println!("{}", &line[idx..]);
        }

        line_index += 1;
    }
    
    Ok(())
}

// Helper for converting something to grep result
pub trait ConvertibleToGrepResult<T> {
    fn to_grep_result(self) -> Result<T>;
}

impl<T, E> ConvertibleToGrepResult<T> for std::result::Result<T, E>
where
    E: Display,
{
    fn to_grep_result(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(FileGrepError(error.to_string())),
        }
    }
}
