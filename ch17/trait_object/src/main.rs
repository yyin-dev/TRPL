#![allow(unused)]
fn main() {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // Box<dyn Draw> is a trait object; a placeholder for any type inside
        // a Box that implements the Draw trait.
        // We must use Box since the size is not known at compile time.
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        // This works differently from defining a struct that uses a generic
        // type parameter with trait bounds. A generic type parameter can only
        // be substituted with one concrete type at a time; whereas trait
        // objects allow for multiple concrete types to fill in at runtime.
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    
    impl Draw for Button {
        fn draw(&self) {
            // code to actually draw a button
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    // This is similar to duck typing. However, Rust checks at compile-time if
    // a type implements a particular method. There's no runtime cost for this.

    // Trait objects perform dynamic dispatch
    // When using generics, Rust performs compile-time code generation so there
    // is no runtime cost for trait bounds. This is static dispatch, where the
    // compiler knows what method you are calling at compile time.
    // For trait bounds, the compiler doesn't know at compile time and must use
    // dynamic dispatch. At runtime, Rust use the pointers inside the trait
    // object to known which method to call. This is a runtime cost. 

    // You can only make object-safe traits into trait objects. A trait is 
    // object safe is all methods defined in the trait have the following two
    // properties:
    // 1. The return type is not Self;
    // 2. There're no generic type parameters.
    // Trait objects must be object safe because once you've used the object, 
    // Rust no longer knows its type at compile time. 
}
