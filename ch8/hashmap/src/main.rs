use std::collections::HashMap;

fn main() {
    // Construct hashmap with insert
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Construct hashmap with `collect`
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // HashMap annotation is needed as it's possible to `collect` into many
    // different data structures. However, Rust can infer about the key type
    // and value type. So you can just write _.
    let scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Hashmap and ownership
    // For types that implement the Copy trait, the values are copied into the
    // hashmap. For owned values like `String`, the values are moved and the
    // hashmap will be the new owner.
    // If we insert references to values into the hashmap, the values won't be
    // moved into the hashmap. The values pointed to by the references must be
    // valid as long as the hashmap is valid. We'll talk about lifetimes later.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Access values in a hashmap using `get`/for
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // returns a reference
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Update a hash map
    // Three choice:
    // 1. Replace old value;
    // 2. Ignore new value;
    // 3. Combine old value and new value.
    let mut scores = HashMap::new();

    // Replace old value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Insert only if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Combine old and new value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
