fn main() {
    println!("Hello, world!");

    let lhs = -100500;
    let rhs = {
        let x = 500;
        x * 201 // this block evaluates to `x * 201`, notice no semicolon at the end - expressions do not end with semicolons
    }; // blocks are expressions and can evaluate to a value

    println!("Sum is: {}", another_function(lhs, rhs)); // the function doesn't have to be defined before it is first used, it just have to be defined somewhere
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    x + y // notice no semicolon at the end; a semicolon would turn this into a statement (that does not return a value); functions `return` the last expression implicitly
}
