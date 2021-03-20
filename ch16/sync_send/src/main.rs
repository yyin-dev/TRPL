// Two concurrency concepts are embedded in the language: the std::marker traits
// Sync and Send.

// Allowing ownership transference with Sync
// The Send marker trait indicates that ownership of the type implement Send
// can be transferred betweem threads. Almost every Rust type is Send, but 
// there're some exceptions, including Rc<T>. Rc<T> is implemented for use
// in single-threaded situations where you don't want to pay the thread-safe
// performace penalty.
// Any type composed entirely of Send types is automatically marked as Send.

// Allowing access from multiple threads with Sync
// The Sync marker trait indicates that it's safe for the type implementing
// Sync to be referenced from multiple threads. So, if &T is Send, then T is 
// Sync. Primitive types are Sync, and types composed entirely from Sync are 
// also Sync.

// Implementing Send and Sync manually requires unsafe Rust.

fn main() {
    println!("Hello, world!");
}
