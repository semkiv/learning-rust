use std::collections::HashMap; // when bringing in `struct`s `enum`s and items other than functions, it's idiomatic to specify the full path to the item
// use std::io::{self, Write}; // importing modules using nested paths; imports `std::io` and `std::io::Write`
// use io::prelude::*; // glob operator; imports everything from `io::prelude`

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
