use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        //Note: Using primitive values when a complex type would be more appropriate
        // is an anti-pattern known as primitive obsession.
        // instead of returning a string slice tuple of Vec<String>
        // we return an owned data structure

        // if args.len() < 3 {
        // Panic macros are suitable for programming logic fails
        // this seems to handle a usage issue so we use a Result instead
        //     panic!("Not enough arguments supplied");
        // }
        if args.len() < 3 {
            return Err("Not enough arguments supplied");
        }
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // is_err() returns true if Result is Err. This means CASE_INSENSITIVE is not set.
        let mut option_index: Vec<usize> = Vec::new();
        for (index, arg) in args.iter().enumerate() {
            if arg.starts_with("-") {
                if arg == "-i" {
                    case_sensitive = false;
                }
            } else {
                option_index.push(index);
            }
        }
        // add a multi-path search.
        let file = args[option_index[option_index.len() - 1]].clone();
        let query = args[option_index[1]].clone();
        Ok(Config {
            query,
            file,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // again, expect is a panic! so we use Result here, to indicate usage error
    // let contents = fs::read_to_string(config.file).expect("File not found!");
    let contents = fs::read_to_string(config.file)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_ignore_case(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// lifetime parameters specify which argument
// lifetime is connected to the lifetime of the return value.
// In this case, we indicate that the returned vector
// should contain string slices that reference slices of the argument contents
// (rather than the argument query).

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }
    res
}

fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line)
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct Tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn ignore_case() {
        let query = "ruST";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_ignore_case(query, contents)
        );
    }
    #[test]
    fn check_configuration() {
        let args = vec![
            "0".to_string(),
            "To".to_string(),
            "poem.txt".to_string(),
            "-i".to_string(),
        ];
        let config_object = Config::new(&args).unwrap();

        assert_eq!(false, config_object.case_sensitive);
        assert_eq!("poem.txt", config_object.file);
        assert_eq!("To", config_object.query);
    }
}
