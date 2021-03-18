#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods are defined within the context of a struct (or an enum or a 
    // trait object), and the 1st parameter is always `self`.
    // Methods can:
    // 1. Take the ownership of self;
    // 2. Borrow self immutably;
    // 3. Borrow self mutably.
    // When will you want a method to take the ownership of an instance? If the
    // methods transforms self into something else and you want to prevent the
    // caller from using the original instance after the transformation.
    //
    // Note that Rust has the "automatic referencing and dereferencing" feature
    // that always matches the method signature for you.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated functions: associated with the struct (but not an instacne).
    // They are not methods. You call them using `::` syntax.
    
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
