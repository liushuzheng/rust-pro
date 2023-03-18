use std::net::{Ipv4Addr, Ipv6Addr};


enum IpAddrKind {
    V4,
    V6,
}


// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// 这个枚举拥有4个内嵌了不同类型数据的变体:
// • Quit没有任何关联数据。
// • Move包含了一个匿名结构体。
// • Write包含了一个String。
// • ChangeColor包含了3个i32值。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        // 方法体可以在这里定义 }
    }
}

fn add() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
}


fn route(ip_type: IpAddrKind) {}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(u32),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(v) => v,
    }
}

fn ma_() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn if_let2() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn ma_1() -> u32 {
    let coin = Coin::Quarter(32);
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(v) => {
            println!("{}", v);
            v
        }
    }
}

fn if_let_else() {
    let mut count = 0;
    // let coin = Coin::Quarter(count);
    let coin =Coin::Nickel;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("{}", count)
    }
}

fn main() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // let m = Message::Write(String::from("hello"));
    // m.call();

    // let x: Option<u32> = Some(2);
    // assert_eq!(x.is_some(), true);

    // let x: Option<u32> = None;
    // assert_eq!(x.is_some(), false);

    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let mut absent_number: Option<i32> = None;
    // absent_number = Some(10);
    // println!("{}", absent_number.unwrap());

    // add();

    // if_let2();

    if_let_else();
}

