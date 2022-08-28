use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input your guess");

        // `let foo = bar;`: create a variable foo, and binds the value of bar to it
        let mut guess = String::new(); // mutable, new() is an assiciated function (static function in C++)

        io::stdin()
            .read_line(&mut guess) // the argument needs to be mutable, so `mut`;
            // `&` means pass a reference, and no copy is created.
            // Like varialbes, reference are immutable by default
            // read_line returns a io::Result. There're a number of results in
            // Rust: generic Result, and specific versions for submodules, like
            // io::Result
            .expect("Failed to read line");
        // expect() method is defined on Result. If Result is not Ok, but is Err,
        // then the program crash, and the string in expect() is displayed.
        // If the Result is Ok, expect() returns the value for calling read_line

        // This `guess` is a shadow of the previous `guess`. This allows reusing
        // the same variable name.
        // trim() removes the newline character '\n'.
        // parse() can parse string into a number of types. Thus, we need to tell
        // Rust about the type: `let guess: u32`. In addition, the comparison of
        // guess and secret_number means that Rust will infer that secret_number
        // should also be type u32.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Invalid input. Error is `{}`. Try again!", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Equivalent: u32::cmp(&guess, &secret_number)
        match guess.cmp(&secret_number) {
            // A match expression is made up of arms. An arms consists of a pattern
            // and the code that should be run if the pattern is matched.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
