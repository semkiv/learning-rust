fn main() {
    general_loop();
    while_loop();
    for_loop();
}

fn general_loop() {
    let mut counter = 0;

    // `loop` returns never type (`!`) which coerces into any type, including `i32` returned by the `break` expression below; this is what makes this construction compile
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break expression can have an optional value that can be returned out of the loop
        }
    };

    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 15, 20, 25, 30];
    // iterating over an array
    for element in a.iter() {
        println!(
            "{} ({})",
            element,
            if element % 2 == 0 { "even" } else { "odd" } // `if-else` block is an expression in Rust and can evaluate to a value
        );
    }

    // going over a list of numbers in reverse order
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
