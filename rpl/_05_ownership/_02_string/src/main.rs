// "hi earthlings"
// -> This is a string literal.
// -> Its value is known at compile-time.
// -> Rust hardcodes it into the final executable.
//
// String::from("hi");
// -> This is a String.
// -> Its value is only known in runtime.
// -> It can dynamically change in runtime.
// -> Rust allocates it on the HEAP memory.

// For heap allocated values, Rust needs to manage
// them somehow. When they're no longer needed,
// it should clean them from memory (to drop'em).
fn main() {
    let mut s = String::from("hi");   // s comes into the scope
                                      // its data is allocated on the heap

    s.push_str(", earthlings!");      // change the s

    println!("{}", s);                // print the s

}                                     // s goes out of scope
                                      // Rust calls the drop fn on s
                                      // that returns the memory used by s
                                      // back to OS
