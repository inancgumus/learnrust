fn main() {
    // BUG!
    // This is a bug because s is empty now.
    // Although the word thinks that "they" ends at the 4th index.
    // So println tries to print the first four bytes of s.
    //
    // let mut s = String::from("they were walking down the street");
    // let word = first_word_try(&s);
    // s.clear();
    // println!("s[0..{}] -> {}", word, &s[0..word]);

    //
    // SLICES
    // Instead of tracking indexes, let's use a slice.
    //
    // A slice isn't a ref to a String. Instead, it's a ref to
    // some portion of a String.
    //
    // USAGE
    // let s = String::from("hello");
    //
    // [starting_index..ending_index]
    //
    // &s[0..len(s)]      // hello
    // &s[1..3]           // el
    // &s[0..]            // hello
    // &s[..len(s)]       // hello
    // &s[..]             // hello
    //
    // INTERNALS
    // Each slice stores the starting position and
    // the length of the slice, which is equal to:
    // ending_index - starting_index
    //
    
    let s = String::from("they were walking down the street");
    let word = first_word(&s);
    println!("{}", word);

    //
    // string literals are slices.
    //
    // we can pass them directly to an fn if it accepts
    // a string slice.
    println!("{}", first_word("hello world"));

    //
    // TRY THIS
    //
    // Rust protects us from syncing errors.
    //
    // let mut s = String::from("they were walking down the street");
    // let word = first_word(&s);    // get a slice to the first word portion of s
                                     // &s means an immutable borrow

    // s.clear();                    // can't clear s because clear needs to borrow
                                     // s mutably.

    // println!("{}", word);         // print a non-existent word?!?

    //
    // ERROR
    // String slice range indices should respect UTF-8
    // character boundaries.
    //
    // let s = String::from("yolun aşağısına doğru yürüyorlardı");
    // println!("{}", &s[6..8]);    
}

/*
fn first_word_try(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            // return the first word
            return i;
        }
    }
    // return the whole word
    s.len()
}
*/

//
// string literals are slices.
//
// -> so it's better to accept a string slice
// -> instead of a String reference.
//
// This way, first_word can accept both a String ref,
// as well as a string literal, and so on.
//
// let s = String::from("hey");
// let w = first_word(&s[..]);  // pass the whole s
//
// let l = "hello";             // a string literal
// let w = first_word(&l[..]);  // pass a string literal
// let w = first_word(l);       // <- or simply.
                                // works because string literals == slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}