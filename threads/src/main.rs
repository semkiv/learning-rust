use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // we spawn a thread by passing a closure to it; closures used with threads usually move the data from their environment to safely pass it
    let handle = thread::spawn(move || {
        for i in v.iter() {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // `thread`s are not joined automatically; we need to call `join` method (return `Err` if the thread panicked) otherwise the thread will be simply canceled as soon as the main thread finishes
}
