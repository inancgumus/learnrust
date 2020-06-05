// 💯 See: _naked.rs for the annotation-free version.

// We need to bring the input/output library into scope,
// So we can accept user input and print the result as output.
// std => Standard Library
use std::io;

// For the stdout().flush() below.
use std::io::Write;

// We want to use the Rng from the rand crate to generate
// Random numbers.
//
// To install this, first you need to go to your Cargo.toml
// file, and add it there, below the [dependencies] section.
use rand::Rng;

// To compare values we'd also need to use the Ordering enum.
//
// Ordering enum contains: Less, Equal, and Greater.
//
// --> ::* at the end returns every value in the Ordering.
//     So we can use them directly like Less, Equal, Greater.
//     Instead of: Ordering::Less, Ordering::Equal, etc.
use std::cmp::Ordering::*;

fn main() {
    // Let's introduce a new immutable variable named secret.
    // We're going to compare this one with the user input: guess.
    //
    // --> Variables are immutable by default.
    //     (Immutable == Unchangeable).
    //
    // --> thread_rng() returns us a random generator that
    //     will be seeded by the current operating system thread.
    // --> gen_range() is a method on the returned random
    //     generator. It generates a random number between 1 and 100.
    let secret = rand::thread_rng().gen_range(1, 101);

    // loops infinitely
    loop {
        print!("guess: ");

        // stdout gets flushed on newlines.
        // print! doesn't contain a newline.
        // So we need to flush stdout ourselves.
        io::stdout().flush().expect("🤬 cannot flush");

        // Let's create a new, empty, and mutable string variable.
        //
        // ==> mut makes the variable mutable.
        //
        // ==> String::new() creates and returns a new String instance.
        // --> The String type is a growable, UTF-8 encoded text.
        // --> :: indicates new is an associated fn of String.
        //     (aka a static method).
        //
        // ==> Type Inference
        // --> Rust automatically guesses the type of the guess variable.
        // --> Since String::new() returns a String, so guess is also
        //     a String.
        let mut guess = String::new();

        // Let's accept user input and put it into the guess variable.
        //
        // ==> io::stdin() calls the stdin fn from the io module.
        // --> Returns a std::io::Stdin instance.
        // --> Stdin represents a handle to the stdin for your terminal.
        io::stdin()
            // Let's read the user input into guess.
            //
            // read_line takes whatever the user types into stdin, and
            // place that into a string, so it takes a string as an
            // argument.
            //
            // --> read_line() calls the read_line method on the Stdin
            //     instance to get input from the user.
            // --> `&mut guess` passes the mutable variable to the
            //     read_line method. So the readline can update it.
            // --> & indicates a reference. This allows read_line
            //     to work with the same guess instance. Prevents
            //     copying on memory. Passes a pointer to it.
            .read_line(&mut guess)
            // Let's handle the error: Could we read the user input?
            //
            // ==> read_line returns an io::Result enum that can either
            //     be Ok, or Err. Ok, and Err are called the variants of
            //     the io::Result enum type.
            //
            //     Ok  --> User input read successfuly.
            //     Err --> Couldn't read the user input.
            //
            // ==> Here we call the expect method of the io::Result.
            //     --> The result of the read_line defines the
            //         behavior of the expect method.
            //     --> If the result is Err: expect() crashes the
            //         program, and prints the error message.
            //     --> If the result is Ok: expect() extracts the
            //         return value from the Ok.
            //
            // ==> If you don't use the expect, `cargo check` will
            //     give you a warning about that you should handle
            //     the error.
            .expect("🤬 cannot read input");

        // Let's convert the user input into an integer.
        //
        // Rust is a strongly statically-typed programming language.
        // --> So you can't compare the user input (a string)
        //     to the secret (an i32—the default 32-bit integer type).
        //
        // ==> Let me explain the code below:
        // --> guess below is a new guess variable with u32 type.
        // --> u32 == a 32-bit unsigned integer.
        // --> trim() removes the spaces around the previous guess
        //     string variable's value.
        // --> parse() returns the string value as an u32 value.
        // --> expect checks if the previous guess's value was an
        //     integer, if not, crashes the program.
        //
        // let guess: u32 = guess.trim().parse()
        //      .expect("Please type a number!");

        // Let's not crash the program, and gracefully handle the err.
        // == match is explained at the end of the program ==
        let guess: u32 = match guess.trim().parse() {
            // 1. if the input is a number, return it.
            Ok(n) => n,

            // 2. if it's not reloop.
            // --> _ is a catch-all value.
            //    Here, it catches every type of err.
            Err(_) => {
                println!("🤬 not a number");
                continue;
            }
        };

        // crab pincers == {} — holds a value in-place.
        // --> println injects values into a string using placeholders.
        // --> {} is a placeholder, and the value is the guess's
        //     value.
        println!("you say: {}, let's see...", guess);

        // Let's compare the user input with the secret number.
        //
        // ==> cmp() method can compare two values that can be compared.
        // --> & means that cmp takes the reference of the secret variable.
        // --> It returns a std::cmp::Ordering value.
        //
        // ==> match expression allows you to compare values.
        match guess.cmp(&secret) {
            Less => println!("🤨 bigger"),
            Greater => println!("🤨 smaller"),
            Equal => {
                println!("🥳 you rock!");
                break; // exit the loop
            }
        }
        println!();
    }
}
