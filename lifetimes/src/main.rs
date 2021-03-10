use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
        s: "Yes, this is me",
    };
    i.announce_and_return_part("gentlemen");
    println!("The level {}", i.level());

    println!(
        "{}",
        longest_with_an_announcement(
            &string1,
            &string2,
            "Wow, this is cool!.. Actually, not so much..."
        )
    );
}

// sometimes Rust compiler can’t tell whether the referenced object is still valid at them moment the reference to it is used
// then it needs the lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // here we state that `x`, `y` and the returned value all must have the same lifetime referenced to as `a`
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
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

// we can specify lifetime annotations on structs too; here we basically state that an instance of `ImportantExcerpt` cannot outlive the reference it holds
struct ImportantExcerpt<'a> {
    part: &'a str,
    s: &'static str, // `'static` lifetime is a special case for references that outlive the entire program execution
}

impl<'a> ImportantExcerpt<'a> {
    // Explicit lifetime annotations are not required because of the first of the first Rust lifetime elision rules
    fn level(&self) -> i32 {
        3
    }

    // Explicit lifetime annotations are not required because of the first of the third Rust lifetime elision rules
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}. {}.", announcement, self.s);
        self.part
    }
}
