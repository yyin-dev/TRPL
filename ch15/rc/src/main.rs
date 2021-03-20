fn main() {
    // Sometimes, a value should have multiple owners. Example: in graph data
    // structures, one node may be pointed to multiple nodes. The node should
    // not be cleaned up until there's no edge pointing to it.
    // Rc<T> is a reference couting smart pointer. The value is cleaned up
    // only when there's no reference to it any more.
    // Rc<T> is only for use in single-threaded scenarios. For multithreaded 
    // program, reference couting is done differently.

    // Two list sharing a suffix
    use std::rc::Rc;
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // instead of taking ownership of a, clone the Rc<List> that a is holding, 
                                    // thereby increasing the reference count from 1 to 2.
    let c = Cons(4, Rc::clone(&a));
    // Rc::clone doesn't make a deep copy of all the data. It only increments
    // the reference count. So it's cheap.
    {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    // Rc<T> only allow sharing data with immutable reference. Mutable reference
    // is not allowed as they violate the reference rules and cause data
    // races. 
    // However, being able to mutate data is useful. Let's see RefCell<T>.
}
