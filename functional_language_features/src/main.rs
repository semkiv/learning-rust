fn main() {
    let do_nothing_variants: Vec<&dyn Fn(u32) -> u32> = vec![ // `Fn` trait is the most relaxed, an instance of a class implementing this trait can only immutably borrow values from the enviroment; similar traits are `FnMut` (mutably borrows the values) and `FnOnce` (moves the values)
        &(|x: u32| -> u32 { x }), // fully annotated closure
        &(|x| { x }), // parameter and return types are optional and can usually be inferred
        &(|x| x), // if a closure contains just one statement, braces enclosing the body are optional too
    ];

    for func in do_nothing_variants {
        println!("{}", func(42));
    }

    let moved_into = move |s| s; // if we want to force the closure to take ownership of the values it uses in the environment, we can use the `move` keyword before the parameter list; this technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread
    let showcase = String::from("Witness this!");
    println!("{}", moved_into(showcase));

    let sum: u32 = CountToFive::new()
        .zip(CountToFive::new().skip(1)) // iterators in Rust are lazy, so nothing is evaluated at this point...
        .map(|(a, b)| a * b)
        .filter(|x| x % 2 == 0) // ...or even here
        .sum(); // `sum` method consumes the iterator and now the evaluation actually takes place and we get the sum of the squared even elements
    println!("{}", sum);
}

struct CountToFive {
    count: u32,
}

impl CountToFive {
    fn new() -> CountToFive {
        CountToFive { count: 0 }
    }
}

impl Iterator for CountToFive {
    type Item = u32; // `Iterator` trait has an associated type `Item` that corresponds to the type of the dereferenced iterator

    fn next(&mut self) -> Option<Self::Item> { // `next` method is the only method of the `Iterator` trait that has to be implemented; most of the others are implemented in terms of this method
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
