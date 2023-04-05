#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::rc_mod::List::{Cons, Nil};
    use super::*;

    #[test]
    fn box_d() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn count() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}

pub mod rc_mod {
    use std::rc::Rc;
    use crate::rc_mod::List::{Cons, Nil};

    pub enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    fn rc_test() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
    }
}