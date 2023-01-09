fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // 索引5会被绑定到变量word上
    s.clear(); // 这里的clear方法会清空当前字符串，使之变为""
    // 虽然word依然拥有5这个值，但因为我们用于搜索的字符串发生了改变，
    //所以这个索引也就没有任何意义了，word到这里便失去了有效性
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
