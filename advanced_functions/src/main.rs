enum Status {
    Value(u32),
    Stop,
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// functions can be passed to other function like function pointers `fn(...) -> ...`
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// closures are DSTs and cannot be returned directly
fn get_status_handler() -> Box<dyn Fn(Status)> {
    Box::new(|status| match status {
        Status::Value(v) => println!("Got status {}", v),
        Status::Stop => println!("Stopping..."),
    })
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let mut list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect(); //  tuple structs and tuple-struct enum variants initializers are actually implemented as functions returning an instance that’s constructed from their arguments; we can use these initializer functions as function pointers
    list_of_statuses.push(Status::Stop);

    list_of_statuses.into_iter().for_each(get_status_handler());
}
