use regex::Regex;
use std::fmt::{Display, Write};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use colored::{self, Colorize};

#[derive(Debug)]
struct Grep {
    pub query: regex::Regex,
    pub file_path: String,
}

struct FileGrepIterator<'a>
{
    grep: &'a Grep,
}

struct GrepLineMatch
{
    line_index:usize
}

impl<'a> Iterator for FileGrepIterator<'a>
{
    type Item = GrepLineMatch;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug)]
struct FileGrepError(String);
type Result<T> = std::result::Result<T, FileGrepError>;

impl Grep {
    fn parse_query(str: &str) -> Result<Regex> {
        match Regex::new(str) {
            Ok(regex) => Ok(regex),
            Err(error) => Err(FileGrepError(error.to_string())),
        }
    }

    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Grep> {
        args.next().ok_or(FileGrepError(
            "Not enough command line paramters".to_string(),
        ))?; // Skip path to executable
        Ok(Grep {
            query: Grep::parse_query(&args.next().ok_or(FileGrepError(
                "Not enough command line paramters".to_string(),
            ))?)?,
            file_path: args.next().ok_or(FileGrepError(
                "Not enough command line paramters".to_string(),
            ))?,
        })
    }
}

fn read_lines<P>(filename: P) -> Result<io::Lines<BufReader<std::fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(filename).to_grep_result()?;
    Ok(BufReader::new(file).lines())
}

fn main() -> Result<()> {
    let grep:Grep = Grep::build(std::env::args())?;

    for (line_index, maybe_line) in read_lines(grep.file_path)?.enumerate() {
        let line = maybe_line.to_grep_result()?;

        let mut last_index = None;
        let mut line_index_str = String::new();
        for rmatch in grep.query.find_iter(&line) {
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
    }
    Ok(())
}

// Helper for converting something to grep result
trait ConvertibleToGrepResult<T> {
    fn to_grep_result(self) -> Result<T>;
}

impl <T, E> ConvertibleToGrepResult<T> for std::result::Result<T, E>
    where E:Display
{
    fn to_grep_result(self) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(FileGrepError(error.to_string())),
        }
    }
}
