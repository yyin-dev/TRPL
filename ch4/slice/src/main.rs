fn main() {
    // A string slice is a reference to a part of a String.
    // Internally, the slice data structure stores a pointer to the starting
    // position of the string data it refers to (on the heap), and the length
    // of the slice.
    // Diagram: https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices
    // The "string slice" is written as `&str`. Note the difference between
    // &String and &str. There're both references. However, &str can refer to
    // a portion of the entire String.
    // I believe a &str is considered as a immutable reference of a String.
    let mut s = String::from("hello world");
    let str_slice: &str = first_word(&s); // This is a immutable ref to String
    println!("The first word is {}", str_slice);
    // s.clear(); // clear() requires mutable reference
    println!("the first word is: {}", str_slice); // the end of the immutable ref

    // String literals are &str

    // Make first_word take &str as paramter
    // Defining a function to take a &str instead of a &String makes it easier
    // to use. If you have a &str, you can pass it directly. If you have a
    // String, you can get a &str from it.
    let s = String::from("hello world");
    let word = first_word(&s[..]);
    let str_literal = "hello world";
    let word = first_word(str_literal);
    let word = first_word(&str_literal[..]); // You can create &str from &str

    // Other slices
    let a = [1, 2, 3];
    let slice: &[i32] = &a[..];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
