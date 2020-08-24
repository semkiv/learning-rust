fn main() {
    println!("Hello, world!");

    println!("Sum is: {}", another_function(-100500, 100500)); // the function doesn't have to be defined before it is first used, it just have to be defined somewhere
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    x + y // notice no semicolon at the end; a semicolon would turn this into a statement (that does not return a value)
}
