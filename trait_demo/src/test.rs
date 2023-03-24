#[test]
fn fn1() {
    let string1 = String::from("abcd");
    let string2 = "abcd";
    println!("{}", string1 == string2);
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: & 'a str, y: & 'a str) -> & 'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn p2(){
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

trait car<T> {
  fn p(t:T);
}

struct bus {

}
