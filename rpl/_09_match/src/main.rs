enum UsState {
    Alabama,
    Alaska,
    // ...
}

// each Coin variant holds a u64 value.
enum Coin {
    Penny(u64),
    Nickel(u64),
    Dime(u64),

    // this one also holds two values: a u64, and a UsState.
    Quarter(u64, UsState),
}

fn main() {
    println!("10 pennys is {} cent(s).", Coin::Penny(10).cents());
    println!("10 nickels is {} cent(s).", Coin::Nickel(10).cents());
    println!("10 dimes is {} cent(s).", Coin::Dime(10).cents());
    println!(
        "10 Alabama Quarters is {} cent(s).",
        Coin::Quarter(10, UsState::Alabama).cents()
    );
    println!(
        "10 Alaska Quarters is {} cent(s).",
        Coin::Quarter(10, UsState::Alaska).cents()
    );

    // ----------------------------------------------------------------------
    // match is exhaustive.
    // -> this means that you need to exercise all the variants of a value.
    let n = 3u8;
    match n {
        0 => println!("zero"),
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // however, _ will match any value.
        // so you can exhaust a match this way.
        // uncomment this and see what happens.
        _ => (), // () is the unit value, literally empty.
    }

    // ----------------------------------------------------------------------
    // In Rust, there isn't a `null` value.
    //
    // However, there is an Option enum which can either be:
    //
    //      Some
    //      None
    //
    // Some: Option value stores data.
    // None: Option value stores nothing.
    //
    // This way, Rust prevents the Billion-Dollar Null value mistake.
    //
    // You always need to use match to look inside the option value,
    // and find out if it has a value inside (Some), or nothing (None).
    //

    let mut n = Some(6);
    match n {
        Some(6) => println!("six"),
        _ => (),
    }

    // However, this can be cumbersome.
    // Especially, if you only want to print if `n` is 6, as above.
    //
    // There is a better way:
    if let Some(6) = n {
        println!("six");
    }
    // `if let` is syntactic sugar for a match that runs code when the value
    // matches one pattern and then ignores all other values.

    // using an else clause to match other values than 6 is also possible.
    n = Some(7);
    if let Some(6) = n {
        println!("six");
    } else {
        println!("not six, it's {}.", n.unwrap_or_default());
    }
}

impl Coin {
    // this method can be used from an instance of the Coin enum.
    fn cents(&self) -> u64 {
        // match an arm depending on the variant of the Coin enum:
        //      Penny, Nickel, Dime, and Quarter.
        // `amount` is being populated from a variant's data.
        match self {
            Coin::Penny(amount) => amount * 1,  // this arm returns 1
            Coin::Nickel(amount) => amount * 5, // this arm returns 5
            Coin::Dime(amount) => amount * 10,  // this arm returns 10

            Coin::Quarter(amount, state) => {
                let c = match state {
                    UsState::Alabama => 15, // this arm returns 15
                    UsState::Alaska => 25,  // this arm return 10
                };
                amount * c
            }
        }
    }
}
