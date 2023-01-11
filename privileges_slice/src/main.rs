fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn t_first() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // 索引5会被绑定到变量word上
    s.clear(); // 这里的clear方法会清空当前字符串，使之变为""
    // 虽然word依然拥有5这个值，但因为我们用于搜索的字符串发生了改变，
    //所以这个索引也就没有任何意义了，word到这里便失去了有效性
}

fn first_word_p(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn t_first_p() {
    let mut s = String::from("hello world");
    let word = first_word_p(&s);
    s.clear(); // 错误
    println!("the first word is : {}", word);
}

fn first_word_t(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn t_first_t() {
    let my_string = String::from("hello world");
    // first_word 可以接收String对象的切片作为参数
    let word = first_word_t(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word 可以接收字符串字面量的切片作为参数
    let word = first_word_t(&my_string_literal[..]);
    // 由于字符串字面量本身就是切片，所以我们可以在这里直接将它传入函数，
    // 而不需要使用额外的切片语法!
    let word = first_word_t(my_string_literal);
}

fn main() {
    // t_first_p();
    t_first_t();
}
