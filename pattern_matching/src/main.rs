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
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loops() {
    let v = vec!['a', 'b', 'c'];

    // in a `for` loop, the pattern is what directly follows the keyword `for` up until the keyword `in`
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_statements() {
    // a `let` statement is also a place where pattern matching takes place; formally it looks like `let PATTERN = EXPRESSION;`
    let (x, _, z, ..) = (1, 2, 3, 4, 5); // we can use `_` (to ignore a single value) or `..`(to ignore the rest)
    println!("x: {}, z: {}", x, z);
}

fn function_parameters() {
    let point = (3, 5);

    // pattern matching happens in function parameters too, be it a regular function or a closure
    let coordinates_printer = |&(x, y): &(i32, i32)| println!("Current location: ({}, {})", x, y);

    coordinates_printer(&point);
}
