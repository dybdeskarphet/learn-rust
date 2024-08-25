use std::borrow::Borrow;

fn main() {
    // ipaddr_enum()
    message_enum();
}

fn message_enum() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Write(text) => println!("{}", text),
                _ => (),
            }
        }
    }

    let msg = Message::Write(String::from("Merhaba Rust!"));
    msg.call();
}

fn ipaddr_enum() {
    /*
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    } */

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    /* let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Home: {}", home.address);
    println!("Home kind: {:?}", home.kind);

    println!("Loopback: {}", loopback.address);
    println!("Loopback kind: {:?}", loopback.kind); */

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);
}
