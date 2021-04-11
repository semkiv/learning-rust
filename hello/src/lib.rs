use std::sync::{Arc, mpsc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Creates a new `ThreadPool`.
    ///
    /// `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// `new` function will panic if `size` is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    /// Designates an executor for `f` from the pool.
    ///
    /// `f` is the closure to execute.
    ///
    /// # Panics
    ///
    /// `execute` function will panic if the internal MPSC channel is broken.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // `'static` as a trait bound means the type does not contain any non-static references, i.e. the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).expect("MPSC channel is broken.");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // we need two separate loops (one for sending messages and another for shutting down the workers) to avoid deadlocking; if we used a single loop to iterate through each worker, on the first iteration a terminate message would be sent down the channel and join called on the first worker’s thread, now if that first worker was busy processing a request at that moment, the second worker would pick up the terminate message from the channel and shut down and we would be left waiting on the first worker to shut down, but it never would because the second thread picked up the terminate message
        for _ in &self.workers {
            self.sender.send(Message::Terminate).expect("MPSC channel is broken.");
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect("Cannot join the thread. Possibly because it panicked.");
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // loop forever asking for new jobs to execute
        let thread = thread::spawn(move || loop {
            let message = receiver.lock()
                .expect(
                    "The mutex is in a poisoned state (possibly another thread holding the mutex has panicked without releasing it.)"
                ).recv().expect("MPSC channel is broken.");

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
