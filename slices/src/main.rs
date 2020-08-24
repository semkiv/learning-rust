fn main() {
    let s = String::from("Hello, world!");
    println!("{}", dumb_first_word(&s));

    let _hello = &s[0..5]; // slices
    let _hello2 = &s[..5]; // 0 can be omitted
    let _world = &s[6..11];
    let _world2 = &s[6..]; // ending index can be omitted too
    let _full = &s[..]; // full slice

    println!("{}", first_word(&s));
    println!("{}", first_word("Another test string.")); // because string literal is already a slice and we take a slice parameter, this compiles

    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..3]; // slices work for arrays too; in this case the type is &[i32]
    println!("{}", arr_slice[0]);
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
