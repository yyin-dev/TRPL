struct MyAddr {}

fn main() {
    // An "enum(eration)"" contains "variants".
    enum IpAddr {
        V4(u8, u8, u8, u8), // you can put any kind of data inside an enum variant
        V6(String),
        MyAddr(MyAddr),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let my_addr = IpAddr::MyAddr(MyAddr {});

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
            match self {
                Message::Quit | Message::Move { x: _, y: _ } => todo!(),
                Message::Write(_) => todo!(),
                Message::ChangeColor(_, _, _) => todo!(),
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T>
}
