fn main() {
    if_let_expressions();
    while_let_conditional_loops();
    for_loops();
    let_statements();
    function_parameters();
}

fn if_let_expressions() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // patterns in Rust can be refutable (such that the matching can fail) and irrefutable (such that always match)
    // `if let` accepts both refutable and irrefutable pattern, but the compiler will warn about the latter because they're usually not much of use
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    // `if let` can be combined with `else if`...
    } else if is_tuesday {
        println!("Tuesday is green day!");
    // ... and `else if let`
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let_conditional_loops() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // `while let` can be used similarly to `if let` - the loop will continue as long as the pattern matches
    // just as `if let` it accepts both refutable and irrefutable patterns with irrefutable ones being warned about by the compiler
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loops() {
    let v = vec!['a', 'b', 'c'];

    // in a `for` loop, the pattern is what directly follows the keyword `for` up until the keyword `in`
    // `for` loops only accept irrefutable patterns
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_statements() {
    // a `let` statement is also a place where pattern matching takes place; formally it looks like `let PATTERN = EXPRESSION;`
    // `let` statements only accept irrefutable patterns
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn function_parameters() {
    let point = (3, 5);

    // pattern matching happens in function parameters too, be it a regular function or a closure
    // function parameters can be only irrefutable patterns
    let coordinates_printer = |&(x, y): &(i32, i32)| println!("Current location: ({}, {})", x, y);

    coordinates_printer(&point);
}
