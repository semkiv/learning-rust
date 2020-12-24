fn main() {
    const INITIAL_VALUE: i32 = 5; // a constant

    let x = INITIAL_VALUE;
    let x = x + 1; // shadowing
    let x = (x * 2).to_string(); // shadowing again and changing the type

    println!("The value of x is: {}", x);
}
