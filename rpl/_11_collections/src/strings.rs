// https://doc.rust-lang.org/book/ch08-02-strings.html

// Rust wants you to put more thought when working with Strings.
// So that you'll be saved from string/character bugs later on.

//
// String Terminology used in Rust:
//
// String literals
// -> Stored in the program’s binary,
// -> and are therefore string slices.
// -> UTF-8 encoded.
//
// str (_usually seen as `&str`_)
// -> This is the only string type in the core language.
// -> It's a string slice.
// -> UTF-8 encoded.
//
// String
// -> Provided by Rust’s stdlib
// -> Growable, mutable, owned, and UTF-8 encoded.
// -> A wrapper over a `Vec<u8>` (_a slice of bytes_).
//
// Other String Types
// -> OsString, OsStr, CString, and Cstr.
// -> Other crates can create their own string types
//    to encode data, or represent data in memory
//    in different ways.
//

// ===================================================================
// ☝️ When Rustaceans refer to “strings” in Rust,
//    they usually mean the String and the string slice &str types,
//    not just one of those types.
// ===================================================================

#[allow(unused)] // see: https://kutt.it/Qh9Jfb
pub fn run() {
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
    // updating
    // ------------------------------------------------------------------------
    let mut s = "hello".to_string();
    let s2 = " world";
    s.push_str(s2); // push_str mutably borrows s2
    s += ", how are you"; //        ___________/
                          //       /
    s.push('?'); //               v
    println!("s: {} - s2: {}", s, s2); // so you can still use it

    // ------------------------------------------------------------------------
    // let's concat two strings
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
    // let's combine multiple strings (2+) using `format!()`
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
    // what about utf-8?
    // ------------------------------------------------------------------------
    // rust counts how many bytes needed to represent a UTF-8 string.
    // -> for example: ü and ı are 2 bytes each.
    println!("len(\"Gunaydin\")           : {} bytes", "Gunaydin".len()); // 8 bytes
    println!("len(\"Günaydın\")           : {} bytes", "Günaydın".len()); // 10 bytes

    // let's count the characters (scalar values) instead.
    println!(
        "\"Günaydın\".chars().count(): {} chars",
        "Günaydın".chars().count()
    ); // 8

    // ------------------------------------------------------------------------
    // let's index a string, think twice.
    // ------------------------------------------------------------------------
    // 🦀 TLDR: "string indexing is a bad idea!"
    //
    // Example:
    //
    //      "Günaydın"[1]
    //
    // ü is two bytes, so why do you want the first byte? it doesn't make sense.
    // bailout!
    //
    // But, why?
    //
    // -> the return type isn't clear: a char? a grapheme cluster? idk.
    // -> to prevent unexpected values.
    // -> to prevent possibly-buggy code.
    // -> to make it possible to guarantee O(1) performance.
    //    -> string indexing is usually O(1).
    //    -> however, often that may not be true for multiple-byte chars.
    // -> to leave the interpretation and storing raw string data to you, the programmer.
    //

    // So what should you do?
    // -> Use proper ranges to get a slice.
    // -> For example: ü is consisting of 2 bytes, within this range: 1..3
    let s = "Günaydın";
    println!("Günaydın[0..1] = {}", &"Günaydın"[0..1]); // G
    println!("Günaydın[1..3] = {}", &"Günaydın"[1..3]); // ü
                                                        // ⭐️ Remember: & is for borrowing.

    // PANIC ERROR: 1..2 is the first byte of ü.
    // -> Rust can't give you a slice like that.
    // println!("Günaydın[0..1] = {}", &"Günaydın"[1..2]);

    // ------------------------------------------------------------------------
    // let's iterate
    // ------------------------------------------------------------------------
    let s = "Günaydın";
    for c in s.chars() {
        println!("{}", c);
    }
    // Behind the scenes: s.chars() calls s.as_bytes().iter() {}

    // If you want to see the individual bytes, use the `s.bytes()` instead.
    for b in s.bytes() {
        println!("{}", b);
    }

    //
    // What does Rust iterate on with the chars() method?
    //
    // -> chars() iterates over Unicode scalar values.
    //    😡 Is it weird? Kinda.
    //
    //    -> Iteration over grapheme clusters may be what you actually want.
    //    -> This functionality is not provided by Rust's standard library,
    //    -> Check crates.io instead.
    //
    // What's a grapheme cluster?
    //
    // -> you can usually think of it as an ordinary alphabet letter.
    // -> a character (_as we call it_) is a very different thing in the eyes of a computer.
    //
    // Not convinced?
    //
    // read this one: http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries
    // and this     : https://en.wikipedia.org/wiki/Character_(computing)
    //
}
