//
// Rust's HashMap is an ordinary map type that you might already know about from other langs.
//
// -> They are stored on the heap memory.
//
// So Rust's hash map is:
//
//      HashMap<K, V>
//
// HashMap is a generic type:
//
//  -> Keys are of the type `K`, and,
//  -> Values are of type `V`.
//

// first, you need to bring it to this scope.
use std::collections::HashMap;

#[allow(unused)]
pub fn run() {
    // ===================================================================
    // let's create a new hash map.
    // ===================================================================
    let mut reviews = HashMap::new();

    // ===================================================================
    // let's add some game reviews.
    //
    // key   => game name (&str)
    // value => review score (floating-point number)
    // ===================================================================
    reviews.insert("Mass Effect", 7.9);
    reviews.insert("Sam and Max: Hit the Road", 9.5);
    reviews.insert("Beneath a Steel Sky", 9.0);
    reviews.insert("Dune", 8.0);

    {
        // or, you can construct it from these vectors.
        let games = vec!["Mass Effect", "Sam and Max: Hit the Road"];
        let scores = vec![7.9, 9.5];

        // now, all you need is to zip them together into a map.
        let mut reviews: HashMap<_, _> = games.into_iter().zip(scores.into_iter()).collect();
        //                       ^  ^        |                  |
        //                       |_/         v                  v
        //    Guesses the key and value     Keys = &str      Values = float
        //          types automatically  <---------====---------------=====
        //             from the vectors

        // ===================================================================
        // ☝️ into_iter()
        //
        //    Above, each `into_iter()` takes ownership of a vector.
        //
        //    So you can no longer use the games, or scores.
        //
        //          let _ = games.get(0).unwrap();
        //          let _ = scores.get(0).unwrap();
        //
        // ===================================================================
    }

    // ===================================================================
    // let's access the values inside a map
    // ===================================================================
    let mass_effect = reviews.get("Mass Effect");
    println!(
        "They rated Mass Effect with a score of {:?}.",
        mass_effect.unwrap() // unwrap is not cool, you'll see why below.
    );

    // ===================================================================
    // let's check for the existence of keys
    // ===================================================================

    //
    // first, you're going to learn what happens if you try to access
    // a non-existing key.
    //
    let ping_pong = reviews.get("Ping Pong");

    //
    // uncomment the following line and see what happens:
    //
    // println!(
    //     "They rated Ping Pon with a score of {:?}",
    //     ping_pong.unwrap()
    // );
    //

    // now let's learn how to prevent panicking.
    if let None = ping_pong {
        println!("I don't know anything about the Ping Pong game.");
    }
    // ☝️ remember, `if let` is syntactic sugar for `match`.
    //    here, we check if ping_pong is one of the variants of
    //    the Option enum.

    // let's check for an existing game.
    let sam_and_max = "Sam and Max: Hit the Road";
    if let Some(score) = reviews.get(sam_and_max) {
        println!(
            "But they also rated {} with a score of {:?}.",
            sam_and_max, score,
        );
    }
    // ☝️ here, we check if there is "some" value in the value returned
    //    by the `reviews.get(sam_and_max)`.
    //
    //    if there is, Rust binds it to the `score` variable.

    // ===================================================================
    // let's iterate over each key/value pair
    // ===================================================================
    for (key, value) in &reviews {
        println!("-> They rated {} with a score of {}.", key, value);
    }
    // ☝️ remember: borrow, do not unnecessarily own.
    //
    //    if you've used reviews above instead of &reviews,
    //    then you'd be owning, not borrowing.
    //
    //    so, you would no longer be able to use reviews.
    //    it would have been dropped after the loop ends.

    // ===================================================================
    // let's overwrite a value
    // ===================================================================
    // you can overwrite a value if you re-insert with the same key again.
    reviews.insert("Mass Effect", 8.5);
    // let's pretty print everything in the map.
    println!("{:#?}", reviews);

    // ===================================================================
    // let's insert a value only if there isn't a key with the same name
    // ===================================================================
    reviews.entry("Ping Pong").or_insert(6.0);
    // adds Ping Pong with a score of 6.0

    reviews.entry("Ping Pong").or_insert(8.0);
    // has no effect, reviews has Ping Pong

    println!("{:#?}", reviews);

    // ===================================================================
    // let's create a program to find word frequencies in a str
    // ===================================================================
    let text = "that that exists exists in that that that that exists exists in";

    let mut freqs = HashMap::new();

    for w in text.split_whitespace() {
        // split_whitespaces() returns an iterator that returns string sub-slices
        // from `text`. they don't re-allocate. rust is as efficient as it goes.

        let c = freqs.entry(w).or_insert(0);
        // c            -> becomes 0 if `w` key doesn't exist on the map.
        // entry(w)     -> returns the entry of a key from the hash map.
        // or_insert(0) -> if the key was absent, inserts the key `w`
        //                 with a value of 0.

        *c += 1;
        // otherwise:
        // -> or_insert() returns the existing entry in the map
        //    as a mutable reference (&mut i32).
        // -> so you need to dereference it to get the i32 value.
        // -> finally, we can increment it.
    }
    // since each c goes out of scope after the loop's scope, the map's
    // values can be used afterwards. otherwise, this wouldn't be possible.
    // remember, rust doesn't allow immutable & mutable borrowers co-exist.

    println!("{:?}", freqs);
}
