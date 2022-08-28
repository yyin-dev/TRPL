use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    {
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        use List::{Cons, Nil};

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    Nil => None,
                }
            }
        }
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }

    // Rust doesn't catch reference couting cycles. Two ways to avoid this:
    // 1. Write code carefully;
    // 2. Reorganize the data structure so that some references express
    // ownerships while some references don't. This is not always possible.

    // Calling Rc::clone increases the strong_count of an Rc<T> by 1, and
    // an Rc<T> is only cleaned up if its strong_count reaches 0. You can also
    // create weak references to the value in Rc<T> by calling Rc::downgrade,
    // which returns a smart pointer of type Weak<T>. Rc::downgrade doesn't
    // increment strong_count, but increments weak_count. Clean-up can happen
    // when weak_count is above 0.
    // Strong references share ownerships, while weak references don't. Thus,
    // weak references won't cause a reference cycle, as any cycle involving
    // some weak references will be broken when the strong references count of
    // values drop to 0.

    // Since the value Weak<T> refers to might have been dropped, you have to
    // call `upgrade` on a Weak<T>, which returns a Option<Rc<T>>.

    #[derive(Debug)]
    struct Node {
        value: i32,
        // Each node should own its child nodes;
        // We also want to share the ownership
        // with variables, so that we can
        // access nodes directly, so Rc<Node>.
        children: RefCell<Vec<Rc<Node>>>,

        // We want to modify the child nodes of each node, so use RefCell.
        // Parent cannot be Rc<T>, otherwise we have reference cycles. A parent
        // should own its children: if a parent is dropped, its children should
        // also be dropped. However, a child node shouldn't own its parent. If
        // a child node is dropped, the parent should still exists. This is
        // the time to use weak reference.
        parent: RefCell<Weak<Node>>,
    }
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // No infinite loop here! Weak<Node> references are printed as (Weak).
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Visualization of reference count
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
