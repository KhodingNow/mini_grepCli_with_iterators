use std::collections::HashMap;

fn main() {

    let my_closure = || println!("This is a closure");
    my_closure();

    let my_closure = |x: i32| println!("{}", x);

    my_closure(4);
    my_closure(8);

    // A little more complex closure..
    
    let my_closure = || {
        let my_number = 90;
        let enye_i_number = 20;
        println!("The two numbers are {} and {}", my_number, enye_i_number);
    };
    my_closure();

    // Closures are special - bcs they can take variables that are outside 
    // the closure even if you only write ||. Therefore we can;

    //fn main() {
        let number1 = 8;
        let number2 = 16;

        let my_closure = || println!("{}", number1 * number2);
        my_closure();

    //}

    // What more can closures do?

    //fn main() {
        let number11 = 2;
        let number22 = 6;

        let my_closure = |x: i32| println!("{}", number11 + number22 + x);
        my_closure(6);

    //fn main() {
        let my_vec = vec![7, 8, 9];

        let fourth = my_vec.get(3).unwrap_or_else(|| {
            if my_vec.get(0).is_some() {
                &my_vec[0]
            } else {
                &0 // otherwise give a &0
            }
        });

        println!("{}", fourth);
    //} 

    let num_vec = vec![1, 2, 3];

    let double_vec = num_vec
        .iter()
        .map(|number| number * 3)
        .collect::<Vec<i32>>();
    println!("{:?}", double_vec);

// Another good example functionality is;

    let num_vec = vec![10, 12, 14];
    
    num_vec
        .iter()
        .enumerate()
        .for_each( |(index, number)| println!("Index number {} has number {}", index, number));
    //    .map(|(index, number)| println!("Index number {} has number {}", index, number));

    // fn main() {
        let some_numbers = vec![0, 1, 2, 3, 4, 5, 6];
        let some_words = vec!["one", "two", "three", "four", "five"];

        let number_word_hashmap = some_numbers
            .into_iter()
            .zip(some_words.into_iter())
            .collect::<HashMap<_, _>>();

        println!("For key {} we get {}", 2, number_word_hashmap.get(&2).unwrap());
    //}

// fn main() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5, 6];
    let some_words = vec!["ohoo", "oHoh", "Injalo", "kanjani", "five", "six"];
    let number_word_hashmap: HashMap<_, _> = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect();
//}

// HELPFUL methods for closures and iterators:

// fn main() {
    let months = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "novermber", "december"];

    let filtered_months = months
        .into_iter()
        .filter(|month| month.len() < 5)

        .filter(|month| month.contains("u"))
        .collect::<Vec<&str>>();
    println!("{:?}", filtered_months);   
    
//}

// Struct - Company without the CEO:

// fn main() {

// struct Company {
//     name: String,
//     ceo: Option<String>,
// }

// impl Company {
//     fn new(name: &str, ceo: &str) -> Self {
//         let ceo = match ceo {
//             "" => None,
//             ceo => Some(ceo.to_string()),
//         }; // ceo is decided, so now we return Self

//         Self {
//             name: name.to_string(),
//             ceo,
//         }
//     }

//     fn get_ceo(&self) -> Option<String> {
//         self.ceo.clone() // Just returns a clone of the CEO (struct is not copy)
//     }
// }

// // fn main {

//     let company_vec = vec![
//         Company::new("Umbrella Corporation", "Unkown"),
//         Company::new("Ovintiv", "Kwenzekani?"),
//         Company::new("Sunrise at Midday", ""),
//         Company::new("Stark Izinto", ""),
//     ];

//     let all_the_ceos = company_vec
//         .into_iter()
//         .filter_map(|company| company.get_ceo()) // filter_map needs Option<T>
//         .collect::<Vec<String>>();
    
//     println!("{:?}", all_the_ceos);
    
    
//}

//}

// fn main() {

struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        }; // ceo is decided, so now we return Self

        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone() // Just returns a clone of the CEO (struct is not copy)
    }
}

// fn main {

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unkown"),
        Company::new("Noqatiko", "Kwenzekani?"),
        Company::new("Sunrise eziNzulwini", ""),
        Company::new("Stark Izinto", ""),
    ];

    let mut results_vec = vec![]; 

    company_vec.iter().for_each(|company| {
        results_vec.push(company.get_ceo().ok_or_else(|| {
            let err_message = format!("No CEO found for {}", company.name);
            err_message
        }))
    });

    for item in results_vec {
        println!("{:?}", item);
    }    
    
//}

}

// impl Config {
//     pub fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         let ignore_case = env::var("IGNORE_CASE").is_ok();

//         Ok(Config {
//             query,
//             file_path,
//             ignore_case,
//         })
//     } // We need clone here bcs we have a slice with String elements in the param args, butthe build function does not own args. 
//     // To return ownership of a Config instance, we had to clone the values from the query and file_path fields oof Config so the Config instance can own its values.
//     // Due to Iterator knwldg, we can change the build function to take ownership of an iterator as its argument instead of borrowing a slice. We'll use the iterator function instead
//     // of the code that checks the length of the slice and indexes into specific locations.
//     // This will clarify what the Config::build function is doing because the iterator will access the value
//     // Once Config::build takes ownership of the iterator and stops using indexing operations that borrow, we can move String values from the iterator into Config rather than calling clone 
//     // and making a new allocation


// // Using ITERATOR trait instead of indexing;
// // Args implements the Iterator trait, we can call the next method on it:

// impl Config {
//     pub fn build(
//         mut args: impl Iterator<Item = String>,
        
//     ) -> Result<Config, &'static str> {
//         args.next();

//         let query = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a query string"),

//         };

//         left file_path = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a file path"),
            
//         };

//         let ignore_case = env::var("IGNORE_CASE").is_ok();

//         Ok(Config {
//             query,
//             file_path,
//             ignore_case,
            
//         })
//     }
// }

// // Making code clearer with Iterator Adapters

// pub fn search<'a>(query: &str, contents: &'a  str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     for line in contents.line() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }
//     results
// }

// // Using Iterator adapter methods - doing so lets avoid having a mutable intermediate results vector
// // removing a mutable state might enable futre enhancements for parallel searching happen because we would'nt have to manage 
// // concurrent access to the results vector.

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     contents
//         .lines()
//         .filter(|line| line.contains(query))
//         .collect()
// }

// }
