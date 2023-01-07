fn change_d() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn string_d() {
    let s1 = String::from("hello");
    let s2 = s1;
    // let s2 = s1;
    // -- value moved here
    // println!("{}, world!", s1);
}

fn clone_d() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn func_privilege() {
    let s = String::from("hello");  //变量s 进入作用域
    takes_ownership(s); //s的值被移动进了函数
    //所以s 从这里开始不在有效
    let x = 5;
    makes_copy(x);////变量x 进入作用域
    // 但由于i32 是copy 所以依然可以在之后继续使用x
    println!("after x is {x}");
    // println!("after s is {s}");
} //x 首先离开作用域，随后是s
// 但由于s 的值已经发生了移动 所以没有什么特别的事情发生

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string)
}//some_string 离开作用域 drop 函数被自动调用
// some_string 随之被释放了

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer)
}

//some_integer 在这里离开了作用域 没有发生特别的事情
fn ptr_d() {
    let s1 = String::from("hello");
    let ptr = &s1;
    let len = calculate_length(ptr);
    println!("The length of '{}' is {}.", s1, len);
    println!("{ptr}")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn borrow() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    // 与变量类似，引用是默认不可变的，Rust不允许我们去修改引用 指向的值。
    // some_string.push_str(", world");
}


fn main() {
    // change();
    // string_d();
    // clone_d();
    // func_privilege();
    // ptr_d();
    borrow();
}

