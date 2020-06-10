fn main() {
    // err: parse cannot know the type of `guess`.
    // let guess = "42".parse().expect("Not a number!");

    // here, we tell rust that guess is an u32 variable.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // ====================================================
    // SCALAR TYPES
    // Represent a single value.
    // ====================================================

    // INTEGERS:
    // i8 to i128 and u8 to u128
    // -> i means signed integer
    // -> u means unsigned integer

    // there are also:
    // isize and usize.
    //
    // rust determines their sizes depending on the
    // computer the compiler runs
    //
    // for example:
    // on a 64-bit computer: isize is i64 and usize is u64.
    // on a 32-bit computer: isize is i32 and usize is u32.

    // INTEGER LITERALS:
    let _ = 1_2_3; // decimal
    let _: i64 = 0xdeadbeef; // hex
    let _ = 0o444; // octal
    let _ = 0b0101_1010; // binary
    let _ = b'I'; // byte

    // run with: cargo r -q --release
    // to see the wrapping behavior.
    //
    // 255 + 1 becomes 0.
    // 1111_1111 + 1 = 0000_0000

    // let mut over: u8 = 255;
    // over += 1;
    // println!("over: {}", over);

    let _ = 2.0; // f64
    let _: f32 = 3.0; // f32
    let _ = true || false; // bool
    let _ = '💯'; // char

    // TUPLES
    let point = (10, 20.5);
    let (x, y) = point;
    println!("x: {}, y: {}", x, y);
    println!("x: {}, y: {}", point.0, point.1);

    // ARRAYS
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday",
        "Friday", "Saturday", "Sunday"];
    println!("days: {}", days.join(", "));
    
    // each element's type is i64
    // and there are 5 elements.
    let _: [i64; 5] = [1, 2, 3, 4, 5];

    // 3 elements, each one is 1: [1, 1, 1]
    let _ = [1; 3];

    hello();
    say(10, 20);

    // {...} is an expression.
    let (x, y) = {
        let x = 1;
        (x + 1, 5) // <- here it returns a tuple: (2, 5)
    };
    say(x, y);
    say(one().0, one().1);
}

fn hello() {
    println!("hello!");
}

fn say(n: i32, m: i64) {
    println!("n: {}, m: {}", n, m);
}

fn one() -> (i32, i64) {
    (1, 2)
}