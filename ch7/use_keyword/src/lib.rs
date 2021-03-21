// Simple rules:
// 1. A child module can access its parent freely with `super`;
// 2. Siblings can access each other freely (in the same level, but not into it);
// 3. A parent cannot access its child unless public;
// The design is that child modules can hide the internal implementation from
// the parent; and they are given the context in which they are called (by
// the parent).

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
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        use super::front_of_house::serving;
        serving::serve_order();
    }

    fn cook_order() {
        use child_mod; // ok; sibling
        // child_mod::in_child_mod(); // not ok; in_child_mod is not public!
    }

    mod child_mod {
        fn in_child_mod() {
            // Child can refer to parent items freely.
            super::cook_order();
        }
    }
}

pub fn eat_at_restaurant() {
    // front_of_house is not public. However, since eat_at_restaurant is 
    // defined in the same module as front_of_house (crate), it can refer
    // to front_of_house - they are siblings. 
    // Remember: siblings can refer to each other freely.
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}