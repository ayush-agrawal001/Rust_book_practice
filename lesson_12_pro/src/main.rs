use std::{env, error::Error, fs, process};
use lesson_12_pro::{search};

fn main() {

    // let args : Vec<String> = env::args().collect(); ////---> Now we will use the iterator method

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run (config : Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    let results = search(&config.query, &contents, config.ignore_case);

    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query : String,
    file_path : String,
    ignore_case : bool
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Dint get the query string")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Dint get the file path")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}