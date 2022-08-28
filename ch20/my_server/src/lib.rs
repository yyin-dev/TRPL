use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    tx: Sender<Task>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            let thread_rx = Arc::clone(&rx);

            // create some threads and store them in the vector
            threads.push(thread::spawn(move || loop {
                println!("Waiting for task...");
                let f: Task = thread_rx.lock().unwrap().recv().unwrap();
                println!("Received task...");
                f();
            }))
        }

        ThreadPool { threads, tx }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + 'static + Send,
    {
        self.tx.send(Box::new(f)).unwrap();
    }
}
