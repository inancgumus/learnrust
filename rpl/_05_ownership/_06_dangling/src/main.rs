// Dangling References
// Rust never gives you a dangling pointer.

fn main() {
    // let refToNothing = dangle();
    // -> think about using the null pointer here! CRASH!
}

/*
fn dangle() -> &String {            // returns a ref to a String
    let s = String::from("hello");
    &s                              // return a ref to s
}                                   // but s drops!
*/

fn _safe() -> String {
    let s = String::from("hello");
    s
}