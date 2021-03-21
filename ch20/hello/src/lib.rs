use std::thread::{JoinHandle, spawn};
use std::sync::{mpsc, Mutex, Arc};

mod errors;
use errors::PoolCreationError;
use std::collections::BinaryHeap;

// mpsc::Sender<T> + Arc<Mutex<mpsc::Receiver<T>>> creates multi-receiver channel!

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
    // Calling join() consumes the value. But in ThreadPool::drop, we only have mutable reference.
    // Trick: Put JoinHandle<()> in an Option, so that you can move it out using a reference.
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = spawn(move || loop {
            // Correct
            let msg = rx.lock().unwrap().recv().unwrap();

            match msg {
                Message::NewJob(j) => {
                    println!("Worker {} got a msg; executing...", id);
                    j();
                },
                Message::Terminate => {
                    println!("Worker {} receives Terminate, exit...", id);
                    break;
                }
            }

            // Wrong
            // Reference: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
            // while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker {} got a job; executing.", id);
            //
            //     job();
            // }
        });

        Worker { id, thread: Some(thread) }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The `size` is the number of threads in the pool.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (tx, rx) = mpsc::channel();
            let rx = Arc::new(Mutex::new(rx));
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&rx)))
            }

            return Ok(ThreadPool{ workers, sender: tx });
        }
        Err(PoolCreationError)
    }

    pub fn execute<F>(&self, f: F)
    where
        // For F, the return type is omitted.
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::NewJob(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate messages...");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers...");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
