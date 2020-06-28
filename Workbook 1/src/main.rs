extern crate rand;
use std::{cmp::Ordering, collections::HashMap, io, fs, fmt::Display, env, process, error::Error};
use rand::Rng;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// This return type is special https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
fn main()  -> Result<(), Box<dyn Error>> {

    // let args: Vec<String> = env::args().collect();

    my::indirect_call();

    let args = vec!["program name".to_string(),"arg1".to_string(), "arg2".to_string()];

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Query {}, Filename {}", config.query, config.filename);

    println!("Longest announcement: {}", longest_with_an_announcement("asdf", "asdfasdf", "doing work"));

    let hm_result = hash_map();

    if hm_result.is_ok() {
        println!("Hash map result is OK");
    } else {
        println!("Error {}", hm_result.unwrap_err());
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);
    println!("s2 is {}", s2);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };



        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    Ok(())
}

fn largest<T>(list: &[T]) -> &T
    where T: PartialOrd
{
    let mut largest = &list[0];

    for item in list {
        if item > &*largest {
            largest = &item;
        }
    }

    &largest
}

//Create PCMU <-> L16 encoder https://doc.rust-lang.org/book/ch13-04-performance.html

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn hash_map() -> Result<bool, String> {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores:  HashMap<_, _> =  teams.into_iter().zip(initial_scores.into_iter()).collect();


    for (k, v) in &mut scores {
        println!("key {}, value {}", k, v);
        *v += 100;
    }

    for (k, v) in &scores {
        println!("key {}, value {}", k, v);
    }

    Err(String::from("The quick brown fox jumped over the lazy dog"))
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");
        
        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();
        
        // We can also use `self` to access another module inside `my`:
        self::cool::function();
        
        super::cool::function();

        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();
        
        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}