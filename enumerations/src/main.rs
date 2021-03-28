#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // data (any kind) can be attached directly to an enum; in that respect they behave similarly to `std::variant` from C++
    V6(String),
}

enum Color {
    Rgb(u32, u32, u32),
    Hsv(u32, u32, u32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

impl Message { // enuns can have implementations just like structs
    fn call(&self) {
        match self {
            // a pattern to destructure an enum corresponds to the data stored in it
            Message::Quit => println!("Quitting"), // enum variants without data can be matched on the literal only, they cannot be destructured any further
            Message::Move { x, y } => println!("Moving to x = {}, y = {}", x, y), // struct-like enums can be destructured just like structs
            Message::Write(msg) => println!("Writing message '{}'", msg), // tuple-like enums are destructured similarly to tuples
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Changing color to rgb({}, {}, {})", r, g, b), // there can be more complex patterns like matching the nested enum values
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Changing color to hsv({}, {}, {})", h, s, v),
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}, loopback: {:?}", home, loopback);

    let msgs = [
        Message::ChangeColor(255, 255, 255),
        Message::Move { x: 42, y: -42 },
        Message::Quit,
        Message::Write(String::from("Hello world!")),
    ];

    for msg in msgs.iter() {
        msg.call();
    }

    // Rust does not have `null`s. Instead there's `Option<T>` enum that can be either `Some(T)` or `None`
    let some_number = Some(42);
    let absent_number: Option<i32> = None;
    print_optional_number(&some_number);
    print_optional_number(&absent_number);
}

fn print_optional_number(x: &Option<i32>) {
    match x {
        None => println!("None"),
        Some(i) => println!("{}", i),
    }
}
