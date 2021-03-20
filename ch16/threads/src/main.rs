// Threads in Rust
// Programming languages implement threads in different ways.
// - Use OS threads by calling APIs provided by OS. This is the 1:1 model, 
// meaning one OS thread per language thread.
// - Language-specific implementation of threads (known as "green threads"). 
// Languages execute green threads in the context of a different number of OS
// threads. This is the M:N model, meaning there are M green threads per N
// OS threads. 
// The two approaches have different advantages and trade-offs. To Rust, the 
// most important trade-off is runtime support. The "runtime" here refers to
// the code included by the language in every binary. Technically, every
// non-assembly language will have some amount of runtime code. Some languages
// are okay with increasing the runtime size to provide more features. In 
// contrast, Rust needs to have nearly no runtime, and must be able to call
// into C to maintain performance. 
// The M:N model requires a larger language runtime. Thus, Rust uses 1:1 model,
// though there are crates that implement M:N threading. 

use std::thread;
use std::time::Duration;

fn main() {
    // To create a new thread, call thread::spawn with closure.
    // Note that the spawned thread will be stopped when the main thread ends,
    // unless you use join().
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Use move closure with threads
    // `move` allows you to use data from one thread in another thread.
    // `move` forces the closre to take ownership of the values it uses in the
    // environment. 
    let v = vec![1, 2, 3];

    // Rust cannot tell how long the spawned thread will run, and whether `v` 
    // will live long enough.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// There are two concurrency models:
// - message passing
// - shared-state
