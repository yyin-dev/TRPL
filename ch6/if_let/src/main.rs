fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // Equivalently, ...
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // The syntax `if let` takes a pattern and an expression separated by an
    // equal sign. It works the same way as `match`.
    // Using `if let` makes the code more concise. However, you lose the
    // exhaustive checking that `match` enforces.
    // Using an `else` after `if let` is the same as `_` in `match`.
}
