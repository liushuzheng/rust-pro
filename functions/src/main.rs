fn main() {
    // if_d();
    let_if();
}

fn print_ex() {
    let before = (1, 2);
    println!("exchange before value is {} {}", before.0, before.1);
    let (x, y) = exchange(before.0, before.1);
    println!("exchange after value is {} {}", x, y);
}

fn exchange(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

fn if_d() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn let_if() {
    let condition = true;
    let number = if condition {
        5
    } else {
        "six"
    };
    println!("The value of number is: {}", number);
}

