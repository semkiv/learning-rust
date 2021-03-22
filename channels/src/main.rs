use std::sync::mpsc; // `mpsc` stands for "multiple producer, single consumer"
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel(); // `channel` method returns a pair of transmitting (commonly referred to as `tx`) and receiving (`rx`) ends of the channel
    let tx2 = tx1.clone(); // the transmitting end can be cloned allowing for multiple producers

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // transmitting end can be moved into a thread; there `send` method can be used to pass the data to the receiving end when ready; the channel becomes invalid when either of its ends gets dropped thus `send` returns `Err` if the receiving end died
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // receiving end can be used as an iterator; alternative `recv` (blocking) or `try_recv` (non-blocking) methods can be used to obtain the sent data
    for received in rx {
        println!("Got: {}", received);
    }
}
