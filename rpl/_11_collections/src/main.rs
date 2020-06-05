// https://doc.rust-lang.org/std/collections/index.html

mod vectors; // make vectors.rs act like the vectors module.

mod strings; // make strings.rs act like the strings module.

mod hashmaps; // make hashmaps.rs act like the hashmaps module.

fn main() {
    println!("*******************************************************************");
    println!("★ VECTORS ★");
    println!("*******************************************************************");
    vectors::run();
    println!("\n");
    println!("*******************************************************************");
    println!("★ STRINGS ★");
    println!("*******************************************************************");
    strings::run();
    println!("\n");
    println!("*******************************************************************");
    println!("★ HASH MAPS ★");
    println!("*******************************************************************");
    hashmaps::run();
}
