fn main() {
    where_pattern_matching_happens();
    pattern_matching_syntax();
}

fn where_pattern_matching_happens() {
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

fn pattern_matching_syntax() {
    matching_literals();
    matching_variables();
    destructuring();
    ignoring_values();
    at_bindings();
}

fn matching_literals() {
    let x = 1;

    // literals can be matched directly
    match x {
        1 => println!("one"),
        2 | 3 | 4 => println!("two, three or four"), // `|` (or) can be used to match either of its operands
        5..=10 => println!("five to ten"), // `..=` can be used to match inclusive ranges; this works only with numeric values and `char` values
        _ => println!("anything"),
    }
}

fn matching_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Some(y) => println!("Matched, y = {:?}", y), // `match` expressions create a new scope and their arms introduce new variables, so `y` here is not `y` in the outer scope; this can be fixed with so-called match guards:
        Some(n) if n == y => println!("Matched, y = {:?}", y), // a match guard is an additional `if` condition specified after the pattern in a `match` arm that must also match, along with the pattern matching, for that arm to be chosen; match guards are useful for expressing more complex ideas than a pattern alone allows
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn destructuring() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let p = Point { x: 0, y: 7, z: 0 };
    let Point { x: a, y: b, z: c } = p; // variable names don't necessarily have to match struct members names
    assert_eq!(a, 0);
    assert_eq!(b, 7);
    assert_eq!(c, 7);

    match p {
        Point { x: 0, y: 0, z: 0 } => println!("At the origin"), // we can use literals when destructuring a struct
        Point { x, y: 0, z: 0 } => println!("On the x axis at {}", x), // there's a shorthand syntax that allows writing just `x` instead of `x: x`
        Point { x: 0, y, z: 0 } => println!("On the y axis at {}", y),
        Point { x: 0, y: 0, z } => println!("On the y axis at {}", z),
        Point { x, y, z } => println!("On neither axis: ({}, {}, {})", x, y, z),
    }

    let ((feet, inches), Point { x, y, z }) = ((3, 10), Point { x: 3, y: -10, z: 5 }); // there can be quite complex patterns
    assert_eq!(feet, 3);
    assert_eq!(inches, 10);
    assert_eq!(x, 3);
    assert_eq!(y, -10);
    assert_eq!(z, 5);
}

fn ignoring_values() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024);

    match numbers {
        // a single value can be ignored completely (no value will be bound to it) by using `_` in its place; a variable whose name starts with an underscore will not emit compiler warnings if unused but the value will still be bound to it (makes the difference when the value is moved into it); `..` can be used to ignore any parts of a value that we haven’t explicitly matched in the rest of the pattern, it has to be unambiguous though so something like `(.., fifth, ..)` wouldn't work
        (first, _, third, _, fifth, .., _last_but_one, last) => {
            println!("Some numbers: {}, {}, {}, {}", first, third, fifth, last)
        }
    }
}

fn at_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7, // `@` lets us create a variable that holds a value at the same time we’re testing that value to see whether it matches a pattern; basically it saves an additional `match` expression that could otherwise be written in the last pattern's arm
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
