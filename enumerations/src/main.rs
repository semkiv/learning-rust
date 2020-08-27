#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // data (any kind) can be attached directly to the enum which is super handy IMO
    V6(String),
}

#[derive(Debug)]
enum Message {
    _Quit,
    _Move {x: i32, y: i32},
    Write(String),
    _ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        println!("Called!");
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}, loopback: {:?}", home, loopback);

    let m = Message::Write(String::from("hello!"));
    m.call();

    // Rust does not have `null`s. Instead there's `Option<T>` enum that can be either `Some(T)` or `None`
    let _some_number = Some(42);
    let _some_string = Some("lorem ipsum");
    let _absent_number: Option<i32> = None;
}
