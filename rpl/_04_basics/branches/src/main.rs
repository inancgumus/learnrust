fn main() {
    // if expressions
    let number = 5;
    if number == 5 {
        println!("number is 5");
    } else if number == 4 {
        println!("number is 4");
    } else {
        println!("number is not 5");
    }

    // let + if
    let on = true;
    let n = if on { 1 } else { 0 };
    println!("n: {}", n);

    // loops
    let mut i = 0;
    loop {
        println!("is universe infinite?");
        i += 1;
        if i == 2 {
            break;
        }
    }

    // loop expressions
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 2 {
            break counter + 1;
        }
    };
    println!("result: {}", result);

    // while loops
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loops
    let a = [1; 3];
    for e in a.iter() {
        println!("e: {}", e);
    }

    // rev()
    for n in (1..4).rev() {
        println!("{}!", n)
    }
    println!("boom!")
}
