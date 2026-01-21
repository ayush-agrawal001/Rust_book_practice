



pub fn search<'a>(query : &str, content :&'a str, ignore_case : bool) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in content.lines() {
        if ignore_case {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        }else {
            if line.contains(query) {
                results.push(line);
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents, false));
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
            search(query, contents, true)
        );
    }
}