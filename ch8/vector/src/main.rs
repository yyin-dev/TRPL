fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Dropping a vector drops its elements
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    // What happens when we hold references to the elements?
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // The rules for reference still hold
    {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        // v.push(6); // Might cause reallocation
        println!("The first element is: {}", first);
    }

    // Iterating over values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
