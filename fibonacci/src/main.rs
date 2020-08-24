fn main() {
    let n: u32 = loop {
        println!("Enter a number:");

        let mut number = String::new();

        std::io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        match number.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("'{}' is not a valid number 🤷‍♂️", number.trim())
        };
    };

    let mut result: u128 = 1;
    let mut prev: u128 = 1;
    let mut prev_prev: u128;

    for _ in 3..n + 1 {
        prev_prev = prev;
        prev = result;
        result = prev + prev_prev;
    }

    println!("{}", result);
}
