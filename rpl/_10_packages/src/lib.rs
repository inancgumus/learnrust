//
// +-----------------+
// |  MODULE         |
// | +-------------+ |
// | | PACKAGE     | |
// | | +---------+ | |
// | | | CRATE   | | |
// | | +---------+ | |
// | +-------------+ |
// +-----------------+
//

// You can use modules to group related definitions together.
// So others can find what they're looking for more easily.
// That's why you should name your modules well.

// `mod` defines a new module named: `front_of_house`.
mod front_of_house {
    // define another module named `hosting` within the front_of_house module.
    //
    // `pub` makes a definition public.
    // -> Without `pub`, a definition is private by default.
    // -> A parent can't learn details about its children.
    // -> A child can learn everything about its parent.
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    // define another module named `serving` within the front_of_house module.
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

//
// The `module tree` of the front_of_house crate:
//
// crate                       <- module name
// └── front_of_house          <- parent module
//     ├── hosting             <- a child of front_of_house
//     │   ├──add_to_waitlist  <- a child of hosting
//     │   └──seat_at_table    <- a sibling of add_to_waitlist
//     └── serving             <- a sibling of hosting
//         ├── take_order      <- ...
//         ├── serve_order
//         └── take_payment
//
// Looks like a directory structure, eh?
// Right.
// You can use modules to organize your code.
// Just like you can use directories to organize your files.

// note: eat_at_restaurant() is a sibling to the front_of_house module.
// -> they can see each other.
// -> that's why you can access front_of_house from this function.
pub fn eat_at_restaurant() {
    // How to reach out to these modules?
    // Absolute path (the recommended way)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // This fn is also in the same crate.
    // So you can skip typing `crate::`.
    front_of_house::hosting::add_to_waitlist();

    // If hosting and add_to_waitlist weren't `public` (marked with `pub`),
    // Then we wouldn't be able to access them here.
}

// -------------------------------------------------------------------------------

// this fn is defined at the crate level (root).
fn serve_order() {}

// this module is also defined at the crate level (root).
mod back_of_house {
    // this fn is defined at the back_of_house module level.
    fn fix_incorrect_order() {
        // it can call another sibling fn.
        cook_order();

        // normally, this one can't reach out to serve_order.
        // because, they are at different levels.
        //
        // to reach it, this fn needs to use `super`:
        //
        // super -> the parent of back_of_house -> crate
        //
        super::serve_order();
    }

    // this fn is a sibling of fix_incorrect_order.
    // it's been defined in the back_of_house module level.
    fn cook_order() {}

    // --------------------------------------------------------------------------

    pub struct Breakfast {
        // You can also declare fields as public.
        pub toast: String,
        // This one is private.
        seasonal_fruit: String,
    }

    impl Breakfast {
        // only with `summer()`, you can create Breakfast here.
        // because: summer is an associated fn that can access to
        // Breakfast internals.
        // To a private field like: seasonal_fruit.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        // see: eat_meal() below.
    }

    // --------------------------------------------------------------------------

    // When you can make an enum public, all of its variants become public.
    // Enums aren't very useful unless their variants are public.
    pub enum Appetizer {
        Soup,  // public
        Salad, // public
    }
    // see: eat_appetizer() below.
}

// --------------------------------------------------------------------------

pub fn eat_meal() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about bread we'd like (toast is mutable & public)
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // `seasonal_fruit` is private, we're not allowed to see or modify it.
    // meal.seasonal_fruit = String::from("blueberries");
}

// this one can't create Breakfast because it cannot access the
// seasonal_fruit (private) field — unlike the summer fn above.
//
// pub fn cant_breakfast(toast: &str) -> back_of_house::Breakfast {
//     back_of_house::Breakfast {
//         toast: String::from(toast),
//         seasonal_fruit: String::from("peaches"),
//     }
// }
//
// Read more by running this command:
// rustc --explain E0063

// --------------------------------------------------------------------------

pub fn eat_appetizer() {
    // Soup and Salad are public so you can access them here.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    // ...
}

// --------------------------------------------------------------------------

pub fn eat_at_restaurant_verbose() {
    //
    // Using the fn like this is kind of cumbersome and repetitive:
    //
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

// There is a better way.
// Let's bring the hosting module to this scope.
use crate::front_of_house::hosting;
//    ^           ^           ^
//    |           |           |
// create    module name   child of
// name                    front_of_house module

pub fn eat_at_restaurant_concise() {
    // And then you can call the definitions as if they're local:
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// --------------------------------------------------------------------------

//
// 😡UNIDIOMATIC:
//
// Don't bring a fn into scope with use.
//
//      use crate::front_of_house::hosting::add_to_waitlist;
//
//
// 🥳 IDIOMATIC:
//
// Bringing the function's parent module into scope with use so
// so we have to specify the parent module when calling the function:
// -> makes it clear that the function isn't locally defined
// -> while still minimizing repetition of the full path.
//
//      use crate::front_of_house::hosting;
//
//      pub fn eat_at_restaurant_concise() {
//          ★ It's clear that add_to_waitlist is an external fn.
//          ★ And it belongs to the hosting module.
//
//          hosting::add_to_waitlist();
//      }
//
//
// However, it is 🥳 IDIOMATIC to bring structs, enums, and other items with use:
//
// This brings Hashmap struct into the scope.
//
//      use std::collections::HashMap;
//
//      fn main() {
//          let mut map = HashMap::new();
//          map.insert(1, 2);
//      }
//
// If there is a clash, this is the 🥳 IDIOMATIC way:
//
//      use std::fmt::Result;                <- Result is still Result
//      use std::io::Result as IoResult;     <- Result becomes IoResult
//

// --------------------------------------------------------------------------

// RUST FACADE PATTERN: RE-EXPORTING
//
// Re-exporting is useful when the internal structure of your code
// is different from how programmers calling your code would think
// about the domain.
//
//      mod front_of_house {
//           pub mod hosting {
//              pub fn add_to_waitlist() {}
//          }
//      }
//      pub use crate::front_of_house::hosting;
//
// This way, external code can reach out to definitions like this:
//
//      hosting::add_to_waitlist
//

// --------------------------------------------------------------------------

// 👉 `std`, the Standard Library crate is also external to any other packages.

// --------------------------------------------------------------------------

// NESTED `use` to clean up large `use` lists.

//
// 😡 Verbose:
//
//      use std::cmp::Ordering;
//      use std::io;
//
// 🥳 Concise:
//
//      use std::{cmp::Ordering, io};
//
// You can use self to refer to the same package.
//
// 😡 Verbose:
//
//      use std::io;
//      use std::io::Write;
//
// 🥳 Concise:
//
//      use std::io::{self, Write};
//
