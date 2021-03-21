// Using a `;' after `mod front_of_house` tells Rust to load the contents of
// the module from another file with the SAME NAME as the module.
// ATTENTION: when you move a module into a file, you should only move the
// BODY of the mod definition. Think about it this way: the file name is
// implicitly the module name. 
// See body_only.rs and entire.rs for example.
// To use a module in another file, use `mod module_name;` to bring it into
// file, and then do `use module_name::xxx`.
mod body_only;
use body_only::foo;

mod entire;
use entire::entire::bar;


// The same works with directory.
// In front_of_house.rs, it brings hosting into scope `pub mod hosting` and
// exposes it as public;
mod front_of_house; 

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        use super::front_of_house::serving;
        serving::serve_order();
    }

    fn cook_order() {}

    mod child_mod {
        fn in_child_mod() {
            super::cook_order();
        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}