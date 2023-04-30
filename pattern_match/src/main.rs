struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    //为了避免写出类 似于let Point { x: x, y: y } = p这样冗余的代码
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // 对模式中的字面量进行解构和匹配
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}


enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn other_enum_match() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => ()
    }
}

fn test_underline() {
    let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{:?}", s);

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}

#[test]
fn t1() {
    let x = 4;
    let y = true;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

enum M {
    Hello { id: i32 },
}
#[test]
fn bound() {
    let msg = M::Hello { id: 5 };
    match msg {
        M::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        }
        M::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        M::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
