// https://doc.rust-lang.org/stable/std/collections/index.html

// `vectors.rs` file will act like `vectors` module here.
// rust searches for vectors.rs in the current directory.
mod vectors;

mod strings;

fn main() {
    println!("VECTORS");
    println!("-------------------------------------------");
    vectors::run_vectors();

    println!("\n");
    println!("STRINGS");
    println!("-------------------------------------------");
    strings::run_strings();
}
