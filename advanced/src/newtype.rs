use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
#[test]

fn t() {
    let w = Wrapper(vec![String::from("hello"),
                         String::from("world")]);
    println!("w = {}", w);
}

fn strt(){
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";

}