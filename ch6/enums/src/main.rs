fn main() {
    // An "enum(eration)"" contains "variants". 
    enum IpAddr {
        V4(u8, u8, u8, u8), // you can put any kind of data inside an enum variant
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // All variants have the same type, which is the enum. You cannot do this 
    // if you define several structs.
    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // annonymous struct
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T>
}

