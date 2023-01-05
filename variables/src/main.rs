fn main() {
    // mut_test();
    // type_guess();
    // char_d();
    // tuple_d();
    // arr_d();
    arr_panic();
}

fn type_guess() {
    let guess: u32 = "43".parse().expect("Not a number!");
    println!("guess number is {}", guess)
}

fn mut_test() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // cannot assign twice to immutable variable `x`
    x = 6;
    println!("The value of x is: {}", x);
}


fn char_d() {
    let c = 'z';
    let z = '¿';
    let heart_eyed_cat = '🥲';
    println!("char is {} {} {}", c, z, heart_eyed_cat)
}

fn tuple_d() {
    let tup = (500, 6.4, 1);
    let (x, y, _) = tup;
    println!("The value of y is: {}", x);

    // destructuring
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("value is {} {} {}", five_hundred, six_point_four, one)
}

fn arr_d() {
    // 为了写出数组的类型，你需要使用一对方括号，并在方括号中填写
    // 数组内所有元素的类型、一个分号及数组内元素的数量
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for x in a {
        print!("{}=>", x)
    }
    println!();
    let a = [3; 5];
    for x in a {
        print!("{}=>", x)
    }
    println!();
}

fn arr_panic(){
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
}