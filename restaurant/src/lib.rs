mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}


fn serve_order() {
    // back_of_house::hosting::add_to_waitlist();
}

mod back_of_house {

    // pub use crate::front_of_house::hosting;
    fn fix_incorrect_order() {

        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
