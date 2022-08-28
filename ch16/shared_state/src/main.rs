use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex is hard as shared-state concurrency is hard, as you must lock and
    // unlock correctly. Rust's type system and ownership rules make it easier.
    {
        let m = Mutex::new(5);

        {
            // `lock` fails if anther thread holding the lock panicked. In that
            // case, no one can acquire the lock, so we use `unwrap`.
            // You must acquire the lock before using the data, since m is
            // Mutex<i32>, not i32. So the type system prevents you from forgetting
            // to lock.

            // The return value can be used as a mutable reference. You can guess
            // that the return value of `lock` is a smart pointer that implements
            // the Deref trait.
            // `lock` returns a smart pointer called MutexGuard, wrapped in a
            // LockResult. MutexGuard implements Deref to point at the inner data.
            // It also implements Drop to releases the lock automatically when a
            // MutexGuard goes out of scope. This prevents you from forgetting to
            // unlock.
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    // Share Mutex<T> between multiple threads
    // - Mutex + `move` doesn't work;
    // - Mutex + `move` + Rc doesn't work, because Rc is not thread safe;
    // - Mutex + `move` + Arc works.
    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    // Similarities between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>
    // `counter` is immutable but we get mutable reference to the value inside.
    // This means Mutex<T> provides interior mutability. Previously we used 
    // RefCell<T> to mutate data inside Rc<T> in single-threaded situation, 
    // here we use Mutex<T> to mutate data inside Arc<T>. 
    // In short, Rc<RefCell<T>> vs Arc<Mutex<T>>.
    // Just like Rc<T> cannot prevent reference cycles, Mutex<T> cannot prevent
    // deadlocks.
}
