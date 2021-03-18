struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // To mutate a field, the entire instance must be mutable.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // If the field name is the same the variable, Rust have the shortcut.
    let email = String::from("my@email.com");
    let user1 = User {
        email,
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Create instances from other instances
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // Tuple structs
    // Tuple structs have the added meaning the struct name provides, but don't
    // have names associated with fields. This is just like named tuples. You
    // might want to do this when you want to give the entire tuple a name, and 
    // differentiate it from other tuples.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // In the User definition above, we use String instead of &str. We want 
    // instances of the struct to own all its data, and for that data to be 
    // valid as long as the struct instance is still valid.
    // It's possible for struct to store references, but that requires the 
    // use of lifetimes. Let's talk about it later in Chapter 10.
}
