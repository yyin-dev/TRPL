// To call a function, we need to know its path in the module tree.
// A path can be an absolute path or relative path.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }

    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::serving::serve_order();
        }
    
        fn cook_order() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}