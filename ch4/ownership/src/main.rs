fn main() {
    // Stack and heap
    // All data on the stack must have fixed size (due to FIFO). Data with
    // unknown size at compile time must be stored on the heap.
    // Program exeucutes faster when using stack, compared with using heap.

    // Ownership rules
    // - Each value in Rust has a variable that’s called its owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // String type
    // Data types we covered before (scalar types and compound types like
    // tuple and array) are all fixed-sized, so they are stored on the stack.
    //
    // String literals are known at compile time (and immutable), so they are
    // put on the stack. String type is allocated on the heap, as its size is
    // unknown at compile time.
    // When allocated on heap, we need to ensure two things happen:
    // 1. The memory is requested from the memory allocator at runtime;
    // 2. The memory is returned to the allocator when we are done using String.
    //
    // When calling String::from, the 1st step is done. The 2nd step is done
    // when the String is out of scope. A special function called `drop` is
    // called to return the memory back to the allocator.
    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("{}", s); // This will print `hello, world!`
                           // drop() is called, to release memory
    }

    // Ways variables and data interact: MOVE
    {
        // A string is internally stored as: {ptr, len, capacity}
        // ptr: pointer to the actual data on the heap
        // len: amount of memory currently used
        // capacity: amount of memory currently allocated
        // This structure is on the stack, but the actual data is on the heap.
        let s1 = String::from("hello");
        let s2 = s1; // The on-stack structure is copied, but the underlying
                     // on-heap data is not. Copying is too expensive.

        // Recall that when a variable goes out of scope, the `drop` function
        // is called to release the memory. Here, if `drop` is called for both
        // `s1` and `s2`, the memory is freed twice - the double free error.

        // Thus, the assignment makes `s1` invalid, and nothing is done when
        // `s1` goes out of scope.
        // This might look like a "shallow copy", as we are only copying the
        // on-stack structure. However, this is NOT a copy, since `s1` is
        // invalidated and cannot be used anymore. This is a MOVE.

        // Rust design choice: never create deep copy automatically.
    }

    // Ways variables and data interact: CLONE
    {
        // You can create a deep copy if you really want to, using `clone()`.
        let s1 = String::from("hello");
        let s2 = s1.clone(); // heap data is copied - expensive!

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // Stack-only data: COPY
    {
        // For on-stack data, there's no difference between shallow copy and
        // deep copy. Copying is cheap.
        let x = 5; // bind the value 5 to x;
        let y = x; // make a copy of the value in x, and bind it to y
                   // This happens because 5 has size known at compile time, and all
                   // data is on the stack.
        println!("x = {}, y = {}", x, y);

        // Rust has a special annotation called the Copy trait that we can place
        // on types like integers that are stored on the stack. If a type
        // implements the Copy trait, an older variable is still usable after
        // assignment. Rust won’t let us annotate a type with the Copy trait if
        // the type, or any of its parts, has implemented the Drop trait. If
        // the type needs something special to happen when the value goes out of
        // scope and we add the Copy annotation to that type, we’ll get a
        // compile-time error.
    }

    // Passing value to a function
    pass_value_to_function();

    // Return value from a function
    return_value_from_function()

    // In short, the ownership of a variable follows the same pattern every 
    // time: assigning a value to another variable moves it. When a variable 
    // that includes data on the heap goes out of scope, the value will be 
    // cleaned up by `drop` unless the data has been moved to be owned by 
    // another variable.
    // Passing values into a function, or returning values from a function is
    // handled the same way as assigning values.

    // To let a function use a value without taking ownership, we have to 
    // make the function return the value. This can be inconvenient. 
    // Let's see references and borrowing as a solution.
}

fn pass_value_to_function() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward
    
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_value_from_function() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}
