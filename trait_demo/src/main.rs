
mod test;

fn largest<T: PartialOrd >(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
//
// fn new_struct() {
//     let integer = Point { x: 5, y: 10 };
//     let f = Point { x: 1.0, y: 4.0 };
//
//     println!("x is {}",integer.x());
//     println!("x is {}",f.x());
//     println!("x is {}",f.distance_from_origin());
// }
//
// fn main() {
//     // let number_list = vec![34, 50, 25, 100, 65];
//     // let result = largest(&number_list);
//     // println!("The largest number is {}", result);
//     // let char_list = vec!['y', 'm', 'a', 'q'];
//     // let result = largest(&char_list);
//     // println!("The largest char is {}", result);
//     new_struct();
// }

use trait_demo::{Summary, Tweet};

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn trait_demo() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know,
people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

fn main() {
    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: 'c' };
    // let p3 = p1.mixup(p2);
    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    trait_demo();
}