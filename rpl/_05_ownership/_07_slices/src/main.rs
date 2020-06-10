fn main() {
    // ========================================================================    
    // ARRAYS
    // -> An array is a contiguous collection of objects of the same type: T.
    // -> The compiler knows an array's size at compile-time.
    // -> Arrays are stack allocated.
    //
    // You can declare an array like this:
    //
    //      let nums: [i32; 3] = [1, 2, 3];
    //                  ^   ^     ^
    //                  |   |     +---------------+
    //                  |   +--------+            |
    //           Type of elements    |            |
    //                        Array's length      |
    //                                     Array's Elements
    //

    // ========================================================================
    // SLICES
    // -> A slice is a reference to a part of a contiguous sequence of
    //    elements.
    // -> It's actually a two-word object:
    //    -> 1st word: A pointer to the original data.
    //    -> 2nd word: The length of the slice.
    //
    // For example:
    // -> A slice can borrow a portion of the array: nums.
    //
    //      let borrowed_array = &nums;
    //          ^
    //          |
    //        &[i32]
    //     an i32 slice
    // 
    //

    // ========================================================================    
    // A STRING SLICE EXAMPLE
    //
    //      let s = String::from("hello"); // s is a string slice

    // ========================================================================
    // SLICING
    //
    //      [starting_index..ending_index]
    //
    //      &s[0..len(s)]      // hello
    //      &s[1..3]           // el
    //      &s[0..]            // hello
    //      &s[..len(s)]       // hello
    //      &s[..]             // hello
    //

    let s = String::from("they were walking down the street");
    let word = first_word(&s);
    println!("{}", word);

    // ========================================================================
    // string literals are slices.
    //
    // we can pass them directly to an fn if it accepts
    // a string slice.
    println!("{}", first_word("hello world"));

    // ========================================================================
    // ERROR
    // String slice range indices should respect UTF-8 character boundaries.
    //
    // let s = String::from("yolun aşağısına doğru yürüyorlardı");
    // println!("{}", &s[6..8]);    

    // ========================================================================
    // If you could borrow immutably and mutably in the same scope, errors like
    // below could happen. Rust doesn't allow this to happen.
    //
    // Comment out to see the error.
    //
    // let mut s = String::from("they were walking down the street");
    // let word = first_word(&s);    // get a slice to the first word portion of s
                                     //
                                     // &s means an immutable borrow
                                     //
    // s.clear();                    // err -> can't clear s because clear needs
                                     // to borrow s mutably.
                                     //
    // println!("{}", word);         // could print a non-existent word!..
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
// -> As in the first_word() above,
//    It's better to accept a string slice
//    instead of a String reference.
//
// -> This way, first_word can accept both a String ref,
//    as well as a string literal, and so on.
//
fn _use_first_word() {
    let s = String::from("hey");
    let _ = first_word(&s[..]);  // pass the whole s

    let l = "hello";             // a string literal
    let _ = first_word(&l[..]);  // pass the string literal as a slice.
    let _ = first_word(l);       // <- or simply pass itself.
                                 // works because string literals == slices
}