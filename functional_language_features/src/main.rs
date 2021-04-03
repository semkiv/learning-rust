fn main() {
    // `Fn` trait is the most relaxed, an instance of a class implementing this trait can only immutably borrow values from the environment; similar traits are `FnMut` (mutably borrows the values) and `FnOnce` (moves the values)
    let do_nothing_variants: Vec<&dyn Fn(u32) -> u32> = vec![
        &(|x: u32| -> u32 { x }), // fully annotated closure
        &(|x| { x }), // parameter and return types are optional and can usually be inferred
        &(|x| x), // if a closure contains just one statement, braces enclosing the body are optional too
    ];

    for func in do_nothing_variants {
        println!("{}", func(42));
    }

    let moved_into = move |s| s; // if we want to force the closure to take ownership of the values it uses in the environment, we can use the `move` keyword before the parameter list; this technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread; note: `move` closures may still implement `Fn` or `FnMut`, even though they capture variables by `move` - this is because the traits implemented by a closure type are determined by what the closure does with captured values, not how it captures them (the `move` keyword only specifies the latter)
    let showcase = String::from("Witness this!");
    println!("{}", moved_into(showcase));

    let sum: u32 = CountToFive::new()
        .zip(CountToFive::new().skip(1)) // iterators in Rust are lazy, so nothing is evaluated at this point...
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0) // ...or even here
        .sum(); // `sum` method consumes the iterator and now the evaluation actually takes place and we get the sum of the adjacent products divisible by three
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
    type Item = u32; // `Iterator` trait has an associated type `Item` that corresponds to the type of the dereferenced iterator; compared to declaring `Iterator` like `pub trait Iterator<T>` this has the advantage that there's only one type substituted instead of `Item` for each type implementing `Iterator`; if it was `pub trait Iterator<T>` we could have implementation like `Iterator<u32> for CountToFive`, `Iterator<String> for CountToFive` etc and would need to provide annotations for each call to `next`

    // `next` method is the only method of the `Iterator` trait that has to be implemented; most of the others are implemented in terms of this method
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
