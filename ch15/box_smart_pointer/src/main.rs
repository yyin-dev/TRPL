fn main() {
    // Box<T> store data on the heap rather than on the stack. What remains on
    // the stack is the pointer to the heap data. There's no other overhead.
    {
        let b = Box::new(5);
        println!("b = {}", b);
    } // When b goes out of scope, both the box (on the stack) and the data
    // it points to (on the heap) are deallocated.

    // Rust needs to know the size of each type at compile time. The type of 
    // a recursive type cannot be determined at compile time. 
    // This is not allowed:
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    // Boxes only provide the indirection and heap allocation; they don't have
    // any other special capabilities (like other smart pointers). They also
    // don't have any other performance overhead (like other smart pointers).

    // Box<T> is a smart pointer because it implements the Deref trait, which
    // allows Box<T> to be treated as references. And also the `Drop` trait.
    // We explore them now.
}
