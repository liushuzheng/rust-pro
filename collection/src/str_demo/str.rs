#[test]
pub fn push_str(){
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}
#[test]
fn add(){
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s  = "hello";
    let s3 = s1 + &s2 + s; // 注意这里的s1已经被移动且再也不能被使用了
    println!("{}",s3)
}
#[test]
fn get_code(){
    let hello = "Здравствуйте";
    // let answer = &hello[0];
}