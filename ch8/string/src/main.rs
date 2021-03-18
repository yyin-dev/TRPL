fn main() {
    // Rust only has one string type in the core language: `str`, which is 
    // usually seen in its borrowed form `&str`. 
    // The `String` type is provided by the Rust standard library rather than
    // coded into the core language. 

    // Rust doesn't allow indexing into String. Reasons:
    // 1. String is internally a wrapper around Vec<u8>. Rust supports UTF-8 
    // encoding, and each "letter" can take variable number of bytes. So, it's
    // not clear what s[0] should return (the first byte? the first letter)?
    // 2. There are multiple ways to interpret strings (a byte value, a
    // character, a grapheme cluster).
    // 3. String indexing is expected to be O(1), but it's hard to guarantee.

    // String slice containing bytes
    {
        let hello = "Здравствуйте";
        let s = &hello[0..4]; // the first 4 bytes
        // let t = &hello[0..1]; // runtime error: '3' takes 2 bytes
    }
    
    // Iterating over string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // In short, strings are complicated.
}
