fn main() {
    // Pass by reference
    // We call references as function parameters "borrowing".
    // Note that references are immutable by default - you cannot change the
    // thing you borrowed.
    pass_by_ref();

    // Pass by mutable reference
    mut_ref();
    // Restriction: you can have only one mutable reference to a particular
    // piece of data in a particular scope.
    // This allows Rust prevent data race at compile time. 

    // Similarly, mutable refernece cannot exist with immutable reference in
    // the same scope. Note that a reference’s scope starts from where it is
    // introduced and continues through the last time that reference is USED.
    // This is ok:
    let mut s = String::from("hello");

    let r1 = &s; // ok
    let r2 = &s; // ok
    println!("{} and {}", r1, r2); // r1 and r2 are no longer used after this point

    let r3 = &mut s; // ok
    println!("{}", r3); // ok

    let r4 = &s; // ok
    // println!("{}, {}", r3, r4); // r3 and r4 overlaps. NOT OK!

    // Rust prevents dangling reference at compile time.

    // Reference rules:
    // 1. At any given time, you can have either one mutable reference or any
    // number of immutable references.
    // 2. References must always be valid.
}

fn pass_by_ref() {
    // A "reference" in Rust is a pointer to the on-stack structure. It refers
    // to the value of `s1` but doesn't own it. So when `s1` goes out of scope,
    // the value it points to is not dropped.
    // https://doc.rust-lang.org/book/img/trpl04-05.svg
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s comes into scope
    s.len()

    // s goes out of scope. Because it does not have ownership of what it
    // refers to, it is not dropped
}

fn mut_ref() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
