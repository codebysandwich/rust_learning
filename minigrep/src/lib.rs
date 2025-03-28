/*
* File       : minigrep/src/lib.rs
* Time       ：2024/12/13 14:41
* Author     ：sandwich
* version    ：V1.0
* Description：
*/
use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // pub fn new(args: &[String]) -> Result<Config, &str> {
    //     if args.len() < 3 {
    //         return Err("Not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let filename = args[2].clone();
    //     // let case_sensitive =
    //     //     env::var("CASE_INSENSITIVE").is_err();
    //     /*
    //     自己的设计，希望分辨CASE_INSENSITIVE所设置的值
    //     */
    //     let mut case_sensitive = true;
    //     if let Ok(val) = env::var("CASE_INSENSITIVE") {
    //         if val == "1" {
    //             case_sensitive = false;
    //         }
    //     }
    //     Ok(Self {
    //         query,
    //         filename,
    //         case_sensitive,
    //     })
    // }

    pub fn new(
        mut args: env::Args,
    ) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            None => {
                return Err("Didn't get a query string")
            }
            Some(arg) => arg,
        };
        let filename = match args.next() {
            None => return Err("Didn't get a file name"),
            Some(arg) => arg,
        };

        let mut case_sensitive = true;
        if let Ok(val) = env::var("CASE_INSENSITIVE") {
            if val == "1" {
                case_sensitive = false;
            }
        }
        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// pub fn search<'a>(
//     query: &str,
//     contents: &'a str,
// ) -> Vec<&'a str> {
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }

// use iterator
pub fn search<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    // use iterator
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
