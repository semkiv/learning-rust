use std::io;
use std::collections::HashMap;

fn main() {
    println!("Enter the list of integers:");

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line!");

    let mut v: Vec<i32> = Vec::new();
    for item in s.trim().split(" ") {
        v.push(item.parse().expect("Failed to parse input!"));
    }
    v.sort();

    println!("Mean: {}, median: {}, mode: {}", mean(&v), median(&v), mode(&v));
}

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for item in v {
        sum += item;
    }

    sum as f64 / v.len() as f64
}

fn median(v: &Vec<i32>) -> i32 {
    v[v.len() / 2]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut freq = HashMap::new();

    for item in v {
        let count = freq.entry(item).or_insert(0);
        *count += 1;
    }

    **(freq.iter().max_by_key(|entry| entry.1).unwrap().0)
}
