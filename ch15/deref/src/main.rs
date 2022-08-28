fn main() {
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = Box::new(x); // The only difference here is that y points to a
                             // copied value of x, rather than x itself

        assert_eq!(5, x);
        assert_eq!(5, *y); // Box<T> is just like a reference
    }

    struct MyBox<T>(T); // tuple struct definition syntax (nameless fields)
    impl<T> MyBox<T> {
        fn new(x : T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // To allow the dereference operator (*), implement the Deref trait.
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // Rust does this: *(y.deref())
    }

    // Deref coercion is a convenience that Rust performs on arguments to 
    // functions and methods. It works only on types that implement the Deref
    // trait. Deref coercion converts a reference to a type that implements the 
    // Deref trait into a reference to another type. I.e. &T -> &U. E.g.
    // &String -> &str
    // 
    // This happens automatically when we pass a reference to a particular
    // type's value as an argument to a function/method that doesn't match
    // the parameter type. A sequence of calls to `deref` method converts the 
    // type provided to the type required.
    fn hello(name: &str) {
        println!("Hello {}", name);
    }

    hello("Rust");
    let boxed = MyBox::new(String::from("Rust in box"));
    hello(&boxed); // `imple Deref for MyBox` provides &MyBox<String> -> &String;
                   // Standard library provides &String -> &str;
                   // Thus we have &MyBox<String> -> &str.
    // Deref coersion makes code much cleaner. Rust analyzes the way to use
    // Deref::deref at compile time, so there's no runtime cost.

    // How Deref Coersion works with mutability?
    // You can use `Deref` trait to override the `*` operator on immutable refs;
    // You can use `DerefMut` trait to overide the `*` on mutable refs.
    // Rust does deref coercion when it finds types and trait implementations in three cases:
    // 1. From &T to &U when T: Deref<Target=U>
    // 2. From &mut T to &mut U when T: DerefMut<Target=U>
    // 3. From &mut T to &U when T: Deref<Target=U>
}
