fn main() {
    // Variables are immutable by default. But it's different from constants.
    // You cannot use `mut` on constants;
    // Variables are declared with `let`, while constants are declared with
    // `const` and must be type annotated;
    // Constants can only be set to constant expressions.

    // Shadowing
    let mut x = 5;
    x += 1;
    println!("{}", x);

    let x = 7;
    println!("{}", x);

    // constant
    const HOUR_IN_SECOND: u32 = 60 * 60;
    println!("One hour = {} seconds", HOUR_IN_SECOND);

    // Rust is a statically typed language, so all types must be known at
    // compile time. The compiler will try to infer, but when it cannot,
    // the type must be annotated. Example: parse().

    // Scalar types: integer, floating-point numbers, boolean, character
    // Compound types: tuple, array

    // Tuple type
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!(
        "The tuple is ({}, {}, {}), OR ({}, {}, {})",
        x, y, z, tup.0, tup.1, tup.2
    );

    // Array type
    // Array in Rust has fixed length.
    // Vector is provided by the standard library that can grow and shrink.
    // An array is a single chunk of memory allocated on the stack.
    // Accessing past the end of an array is a runtime error.
    let a: [u32; 4] = [0, 1, 2, 3];
    println!("{:?}", a);
    let a = [1024; 5];
    println!("{:?}", a);
    println!("{}", a[0]);
}
