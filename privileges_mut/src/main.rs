fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn borrow_mut() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("the change sting is {}", s);
}

fn error_borrow_twice() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // 由于 r1 在这里离开了作用域，所以我们可以合法地再创建一个可变引用
    // let r1 = &mut s;
    let r2 = &mut s;
    // change(r1);
    // • 两个或两个以上的指针同时访问同一空间。
    // • 其中至少有一个指针会向空间中写入数据。
    // • 没有同步数据访问的机制。
    change(r2);
    println!("the change sting is {}", s);
}

fn dangle() -> &String { //  dangle会返回一个指向String的引用
    let s = String::from("hello"); // s被绑定到新的String上
    &s  // 我们将指向s的引用返回给调用者
    // 变量s在这里离开作用域并随之被销毁，它指向的内存自然也不再有效。
}

// 悬垂引用
fn overhang_reference() {
    let reference_to_nothing = dangle();
}

fn main() {
    // borrow_mut();
    // error_borrow_twice();
    overhang_reference();
}
