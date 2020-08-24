fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // `s1` is invalidated (moved-from) here; Rust moves non-primitive objects by default
    let mut s3 = s2.clone(); // to get a copy of an object `clone` method can be used
    s3.push_str(", world!");

    println!("s2 = {}, s3 = {}", s2, s3);

    let x = 42;
    moves(s3); // `s3` is no longer valid past this point
    copies(x); // `x` however still is
    println!("{}", x);

    let s4 = String::from("Test");
    let (s4, l) = weird_calculate_length(s4);
    println!("s4 = {}, length = {}", s4, l);
}

fn moves(some_string: String) {
    println!("{}", some_string);
}

fn copies(some_integer: i32) {
    println!("{}", some_integer);
}

fn weird_calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
