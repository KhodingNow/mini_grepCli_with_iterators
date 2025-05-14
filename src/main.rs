fn main() {

    println!("Hello, world!");
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    } // We need clone here bcs we have a slice with String elements in the param args, butthe build function does not own args. 
    // To return ownership of a Config instance, we had to clone the values from the query and file_path fields oof Config so the Config instance can own its values.
    // Due to Iterator knwldg, we can change the build function to take ownership of an iterator as its argument instead of borrowing a slice. We'll use the iterator function instead
    // of the code that checks the length of the slice and indexes into specific locations.
    // This will clarify what the Config::build function is doing because the iterator will access the value
    // Once Config::build takes ownership of the iterator and stops using indexing operations that borrow, we can move String values from the iterator into Config rather than calling clone and making a new allocation


// Using ITERATOR trait instead of indexing;
// Args implements the Iterator trait, we can call the next method on it:

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
        
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),

        };

        left file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
            
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            
        })
    }
}

// Making code clearer with Iterator Adapters

pub fn search<'a>(query: &str, contents: &'a  str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.line() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// Using Iterator adapter methods - doing so lets avoid having a mutable intermediate results vector
// emoving a mutable state might enable futre enhancements for parallel searching happen because we would'nt have to manage 
// concurrent access to the results vector.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

}
