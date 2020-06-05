// https://doc.rust-lang.org/book/ch08-02-strings.html

// String literals
// -> Stored in the program’s binary,
// -> and are therefore string slices.
// -> UTF-8 encoded.

// str
// -> This is the only string type in the core language.
// -> It's a string slice.
// -> UTF-8 encoded.

// String
// -> Provided by Rust’s stdlib
// -> Growable, mutable, owned, and UTF-8 encoded.

// Other String Types
// -> OsString, OsStr, CString, and Cstr.
// -> Other crates can create their own string types
//    to encode data, or represent data in memory
//    in different ways.

// ===================================================================
// ☝️ When Rustaceans refer to “strings” in Rust,
//    they usually mean the String and the string slice &str types,
//    not just one of those types.
// ===================================================================

#[allow(unused)] // see: https://kutt.it/Qh9Jfb
pub fn run_strings() {
    // ------------------------------------------------------------------------
    // let's create a new empty String
    // ------------------------------------------------------------------------
    let mut s = String::new();

    // ------------------------------------------------------------------------
    // let's init another s with data
    // ------------------------------------------------------------------------
    let data = "initial data";
    let s = data.to_string(); // converts to a String.
                              // if the type implements the Display trait.

    // ------------------------------------------------------------------------
    // let's init it using a string literal
    // ------------------------------------------------------------------------
    let s = "initial data".to_string();
    // or you can use the `from` fn, it's the same with the one above
    let s = String::from("initial data");

    // ------------------------------------------------------------------------
    // ☝️ many ops available with Vec<T> are available with String as well.
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    // updating a string
    // ------------------------------------------------------------------------
    let mut s = "hello".to_string();
    let s2 = " world";
    s.push_str(s2); // push_str mutably borrows s2
    s += ", how are you"; //        ___________/
                          //       /
    s.push('?'); //               v
    println!("s: {} - s2: {}", s, s2); // so you can still use it

    // ------------------------------------------------------------------------
    // string concat
    // ------------------------------------------------------------------------
    let hello = "hello".to_string();
    let world = " world!".to_string();
    //
    // hello moves below, so it cannot be used again.
    //
    // this is because, the op below uses the add method of `hello`.
    // that method takes ownership of `hello`, and borrows `world`.
    //
    let hello_world = hello + &world;
    //
    // that's why you can no longer use hello.
    //
    //      println!("{} {}", hello, world); // error: 0382
    //
    // this happens in the name of efficiency.
    // -> add method COPIES `world` to `hello`'s buffer.
    // -> so it REUSES `hello`'s buffer to prevent creating a new string
    //    each time you concat a string to it.

    // ------------------------------------------------------------------------
    // combine multiple strings (2+) using `format!()`
    // ------------------------------------------------------------------------
    // -> format doesn't take any ownership.
    // -> it just prints the contents to the screen.
    // -> it doesn't make any concatenation.
    let tic = "tic".to_string();
    let tac = "tac".to_string();
    let toe = "toe".to_string();
    let tic_tac_toe = format!("{}-{tac}-{}", tic, toe, tac = tac);
    // let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe); // same as above
    println!("{}", tic_tac_toe);

    // ------------------------------------------------------------------------
    // string indexing
    // ------------------------------------------------------------------------
}
