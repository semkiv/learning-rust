fn main() {
    let mut counter = 0;

    // general loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break expression can have an optional value that can be returned out of the loop
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    // while loop
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    // for loop iterating over an array
    for element in a.iter() {
        println!("{}", element);
    }

    // for loop going over a list of numbers in reverse order
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
