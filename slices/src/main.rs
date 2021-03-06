fn main() {
    let s = String::from("Hello, world!");
    assert_eq!(dumb_first_word(&s), 6);

    let hello = &s[..5]; // this is a string slice (`&str`); their range is specified as [stating_index..ending_index]; if starting_index is 0 it can be omitted
    let world = &s[7..]; // ending index can be omitted too in which case length will be used instead of it
    println!("{}, {}", hello, world);

    println!("{}", first_word(&s)); // `&String` is implicitly convertible `&str`
    println!("{}", first_word("Another test string.")); // because a string literal is a slice this works straightaway

    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..3]; // slices work for arrays too; in this case the type is &[i32]
    assert_eq!(arr_slice[1], 3);
}

fn dumb_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str { // this function takes and returns a string slice
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
