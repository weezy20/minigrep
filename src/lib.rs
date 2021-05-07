use colored::*;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    /// Query to be searched
    #[structopt()]
    query: String,

    /// File to be searched
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    /// Ignore Case
    #[structopt(short, long = "ignore-case")]
    ignore_case: bool,
}

pub fn run(opt: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(&opt.file)?;
    match search(opt, &file_contents) {
        Some(result) => {
            for res in result {
                println!("{}", res.bold());
            }
        }
        None => println!("Match not found"),
    }
    Ok(())
}

fn search<'a>(opt: &Config, content: &'a String) -> Option<Vec<&'a str>> {
    let mut vec = vec![];

    match opt.ignore_case {
        true => {
            for line in content.lines() {
                if line.to_lowercase().contains(&opt.query.to_lowercase()) {
                    vec.push(line);
                }
            }
        }
        false => {
            for line in content.lines() {
                if line.contains(&opt.query) {
                    vec.push(line);
                }
            }
        }
    }

    if vec.len() > 0 {
        Some(vec)
    } else {
        None
    }
}
