fn main() {
    // Difference between statements and expressions in Rust:
    // Statements are instructions that perform some action and do not return
    // a value. Expressions evaluate to a resulting value.

    // Making a function call is an expression; a numeric literal is an
    // expression; { ... } is an expression.
    let y = {
        let x = 3;
        x + 1
        // Expressions don't include ending semicolons. Adding a semicolon
        // to the end of an expression turns it into a statement.
    };
    println!("{}", y);

    // Expression as return value example
    println!("Five: {}", return_five());

    // using `if` in a `let` statement
    let number = if true { 5 } else { 6 };
    println!("{}", number);

    // Return values from loop: add the value after the `break` keyword
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // Exercise
    println!("Fib(10) = {}", fib(10));
}

fn return_five() -> i32 {
    // The last expression is implicitly the return value
    5
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}
