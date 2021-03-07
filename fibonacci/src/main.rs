fn main() {
    let n: u32 = loop {
        println!("Enter the index:");

        let mut index = String::new();

        std::io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index = index.trim();

        match index.parse() {
            Ok(idx) => break idx,
            Err(_) => println!("'{}' is not a valid index 🤷‍♂️", index),
        };
    };

    let mut result = 0u128;
    let mut prev = 1;
    let mut prev_prev;

    for _ in 0..n {
        prev_prev = prev;
        prev = result;
        result = prev + prev_prev;
    }

    println!("{}", result);
}
