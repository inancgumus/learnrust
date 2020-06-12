// Convert strings to pig latin (_Keep in mind the details about UTF-8 encoding!_).
fn main() {
    println!("{:10} -> {}", "apple", pigify("apple"));
    println!("{:10} -> {}", "first", pigify("first"));

    // let's pigify for multiple words
    let s = "first consonant of each word is moved to the end of the word";

    // create a string with `s.len() * 2` capacity to prevent
    // memory fragmentation while adding new substrings.
    let mut ns = String::with_capacity(s.len() * 2);

    // split s to words, pigify, and add each one to `ns`.
    let mut words = s.split_whitespace();
    while let Some(w) = words.next() {
        ns.push_str(pigify(w).as_str());
        ns.push(' ');
    }
    println!("\n{}\n{}", s, ns);
}

// let's first make it work for a single word.
//
// pigify returns an owned String because
// it doesn't have to retain ownership
// to the returned String.
fn pigify(w: &str) -> String {
    // get an utf-8 character iterator
    let mut chars = w.chars();

    // get the first utf-8 char, and move the iterator to the next char.
    let first: char = match chars.next() {
        Some(c) => c,                  // if `w` isn't empty, `first` is `c`
        None => return "".to_string(), // otherwise, return "" from pigify()
    };
    // we use match here because `next()` returns an Option.

    // transform the word
    match first {
        // if the word starts with a vowel:
        //
        //    "apple" becomes "apple-hay"
        //
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", w),
        // otherwise:
        //
        //    "first" becomes "irst-fay"
        //
        // -> chars points to the 2nd char and forward.
        //    because we moved it near the start of pigify().
        //
        // -> as_str() returns a &str (a string slice) for that portion.
        //
        _ => format!("{}-{}ay", chars.as_str(), first),
        //
        // requirement was handling utf-8 properly.
        // so you shouldn't do something like this: &w[1..]
        // -> why? the first character could be multiple-bytes.
        //
    }
}
