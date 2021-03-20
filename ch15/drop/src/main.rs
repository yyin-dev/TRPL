fn main() {
    // The Drop trait allows you to customize what happens when a value is 
    // about to go out of scope.
    struct CustomSmartPointer {
        data: String,
    }
    
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    // Variables are dropped in the reverse order of creation.
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }
    

    // You cannot call Drop::drop manually. Use `std::mem::drop` function.
    // The reason is that Rust still calls `drop` on the value when it goes
    // out of scope. So this can cause double free.
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
