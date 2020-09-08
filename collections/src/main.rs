use std::collections::HashMap;

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Testing vector...");
    test_vector();
    println!("Testing string...");
    test_string();
    println!("Testing hash map...");
    test_hashmap();
}

fn test_vector() {
    let v1: Vec<i32> = Vec::new(); // an empty vector
    println!("v1 is {:?}", v1);

    let v2  = vec![1, 2, 3]; // a vector with initial values created with the `vec!` macro; the value type is inferred to be `i32`
    println!("v2 is {:?}", v2);

    let mut v3 = Vec::new(); // another empty vector, mutable this time; value type is inferred from further use
    v3.push(4);
    v3.push(5);
    v3.push(6);
    println!("v3 is {:?}", v3);
    for i in &mut v3 { // iterating over the elements
        *i *= 10;
    }
    println!("v3 is {:?}", v3);

    let v4 = vec![1, 2, 3, 4, 5];
    let third : &i32 = &v4[2]; // elements can be accessed by index; `[]` panics when index is out of bounds
    println!("The third element is {}", third);

    match v4.get(99) { // `get` method is another way of accessing elements in a vector; it doesn't panic and returns `None`
        Some(third) => println!("The 100th element is {}", third),
        None => println!("There is no 100th element."),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
    println!("Spreadsheet row: {:?}", row);
}

fn test_string() {
    let s1 = "intial_contents".to_string(); // `to_string` method works with any type that implements `Display` trait, including string literals
    println!("{}", s1);

    let mut s2 = String::from("Привіт"); // `String` is capable of storing Unicode characters
    s2.push_str(", світ"); // `push_str` method appends a string slice
    s2.push('!'); // `push` method appends a single character
    println!("{}", s2);

    let s3 = String::from("Hello");
    let s4 = String::from(", world!");
    let s5 = s3 + &s4; // operator `+` is weird: it works as `fn add(self, rhs: &str) -> String`, so it invalidates the left operand and leaves intact the right one
    println!("{}", s5);

    let tokens = ("tic", "tac", "toe");
    let s6 = format!("{}-{}-{}", tokens.0, tokens.1, tokens.2);
    println!("{}", s6); // formatting using `format!` macro

    let weird_unicode_string = String::from("नमस्ते");
    println!("Contents as bytes:");
    for item in weird_unicode_string.bytes() { // `bytes` method returns raw representation in memory
        println!("{}", item);
    }
    println!("Contents as chars:");
    for item in weird_unicode_string.chars() { // `chars` method treats Unicode symbols correctly, but some of them might be not proper symbols (diacritics, for instance)
        println!("{}", item);
    }
    // println!("{}", &weird_unicode_string[0..2]); // simple slice may panic if index is not at the character boundary (it takes 4 bytes to encode a character from the given string)
}

fn test_hashmap() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![0, 0];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(intial_scores.into_iter()).collect(); // there's no handy macro to create a hash map so here we are building one using `zip` method on an iterator and `collect` method on a vector of tuples that `zip` generates; the type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures and Rust doesn’t know which we want unless we specify, however for the parameters for the key and value types, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors
    println!("{:?}", scores);

    let yellow_team = String::from("Yellow");
    let blue_team = String::from("Blue");
    scores.insert(yellow_team, 10); // inseting into a `HashMap` overwrites the existing values and invalidates the non-Copy types; `yellow_team` is invalid past this point
    scores.insert(blue_team, 50); // inseting into a `HashMap` overwrites the existing values and invalidates the non-Copy types; `blue_team` is invalid past this point
    match scores.get("Blue") { // `get` method returns None if there's no such key
        Some(&score) => println!("Blue team score is {}.", score),
        None => println!("There's no Blue team!"),
    }
    scores.entry(String::from("Yellow")).or_insert(50); // `entry` method queries for a key and returns `Entry` enum whose `or_insert` method does insertion only if the key was not found
    for (key, value) in &scores { // iterating over a HashMap
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0); // updating the value based on the current value; `or_insert` returns a mutable reference to the existing value (if there was one) or to the newly inserted one
        *count += 1;
    }
    println!("{:?}", word_count);
}
