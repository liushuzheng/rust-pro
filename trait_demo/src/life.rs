struct ImportantExcerpt<'a> {
    part: &'a str,
}
#[test]
fn new() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

#[test]
fn split(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.');
    for x in first_sentence {
        println!("value is {}",x);
    }
}