use std::cell::RefCell;
use std::rc::Rc;

// Interior mutability is a design pattern in Rust that allows you to
// mutate data even when there're immutable references to that data; this
// is disallowed by the borrowing rules.
// To mutate data, the pattern uses `unsafe` code inside a data structure
// to bend Rust's usual rules governing mutation and borrowing.

// RefCell<T> represents single ownership over the data it holds. With
// references and Box<T>, the borrowing rule's invariant are enforced at
// compile time:
// - At any time, you can have either (but not both of) one mutable
// reference or any number of immutable references;
// - References must always be valid.
// With RefCell<T>, these invariants are enforced at runtime. With
// references, if you break the rule, you get compile eroor. With
// RefCell<T>, if you break the rule, your program will panic.

// The advantage of compile-time borrow check is to catch errors early
// in the compile time, so there's no runtime overhead.
// The advantage of runtime borrow check is to allow certain memory-safe
// scenarios, which were disallowed by compile-time checks. Static analysis
// by the Rust compiler is inherently conservative. If it cannot analyze
// some (possibly correct) code, it rejects it.
// If Rust accepts an incorrect program, then users cannot trust Rust; but
// if Rust rejects a correct program, the programmer will be inconvenienced,
// but nothing catastrophic will happen.
// RefCell<T> is useful when you're sure that your code follows the borrowing
// rules but the compiler is not able to understand and guarantee that.
// Similar to Rc<T>, RefCell<T> is for single-threaded program.

// Recap:
// - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T>
// have single owners.
// - Box<T> allows immutable or mutable borrows checked at compile time;
// Rc<T> allows only immutable borrows checked at compile time; RefCell<T>
// allows immutable or mutable borrows checked at runtime.
// - Because RefCell<T> allows mutable borrows checked at runtime,
// you can mutate the value inside the RefCell<T> even when the RefCell<T>
// is immutable.

// Context: we want to test method LimitTracker::set_value. Thus, we need
// a "mock object" that implements Messenger trait.
pub trait Messenger {
    fn send(&self, msg: &str); // immutable reference
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    // MockMessenger is the mock object.
    struct MockMessenger {
        // Store `send_messages` in a RefCell<T>, and `send` can mutate it.
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // borrow_mut returns a mutable reference to the value inside 
            // RefCell<Vec<String>>, even though RefCell<T> is immutable.
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    // How this works?
    // Methods `borrow` and `borrow_mut` are part of the safe API of RefCell<T>.
    // The `borrow` methods return a Ref<T>, and `borrow_mut` returns a 
    // RefMut<T>. Both types implement Deref.
    // At runtime, the RefCell<T> keeps track of how many Ref<T> and RefMut<T>
    // smart pointers are currently active. When `borrow` is called, the
    // RefCell<T> increments its count on immutable borrows; When a Ref<T> value
    // goes out of scope, the count of immutable borrows goes down by one. Just
    // liek compile-time borrowing rules, RefCell<T> allows having many
    // immutable borrows or one mutable borrow at any time.
    // If the rule is violated, the program will panic. 
    // Two implications of catching borrowing errors at runtime: 
    // 1. You may find a mistake later in the development process, and possibly
    // not until the code is deployed;
    // 2. A small runtime performance penalty.
}

fn main() {
    // A common way to use RefCell<T> is in combination with Rc<T>. With Rc<T>,
    // you can have multiple owners with immutable access to the same data. 
    // If you have a Rc<T> that holds a RefCell<T>, you can have multiple
    // owners AND each can mutate the data.
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));


    // 3 -+
    //    v
    //    5 -> 6
    //    ^
    // 4 -+

    *value.borrow_mut() += 10;

    println!("suffix after = {:?}", value);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
