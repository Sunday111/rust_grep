use colored::{self, Colorize};
use regex::Regex;
use std::fmt::{Display, Write};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let mut reader = BufReader::new(file);
    let mut line_buffer = String::new();
    let mut line_index:u32 = 0;

    while reader.read_line(&mut line_buffer).to_grep_result()? > 0 {
        let line = line_buffer.trim_end_matches('\n');
        let mut last_index = None;
        let mut line_index_str = String::new();
        for rmatch in pattern.find_iter(&line) {
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
        line_buffer.clear();
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
