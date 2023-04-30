use object_oriented::{Button, Screen, SelectBox};
use object_oriented::sp::another::Post;
use object_oriented::sp::stat;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();


    let screen = Screen {
        components: vec![
            Box::new(String::from("Hi")),
        ],
    };
    screen.run();
}

fn st(){
    let mut post = stat::Post::new();
     post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
#[test]
fn another(){
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
