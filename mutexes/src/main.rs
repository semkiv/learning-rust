use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // `Mutex<T>` in Rust is a smart pointer whose internal value is accessible through `lock` method; `Mutex<T>` cannot be shared among the threads directly so we need wrap it into a smart pointer; `Rc<T>` is not thread safe but it has an atomic alternative - `Arc<T>`
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // in order to share `counter` among the threads we need to clone it (similarly to `Rc::clone`)
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // `lock` fails if another thread holding the lock has panicked; `Mutex<T>` follows the idea of interior mutability (similarly to `RefCell<T>`) so its internal value can be mutated even though the mutex itself is immutable

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
