fn main() {
    let x = 5;
    println!("x: {}", x);
    
    // x variable is immutable.
    // so we can't change its value.
    // x = 6;
    
    // but we can declare a new variable,
    // with the same name: `x`.
    //
    // this way, we don't need to imagine
    // new variable names.
    let x = 6;
    println!("x: {}", x);
    
    // we can also create a constant.
    // constant are compile-time values.
    // so, anywhere MAX_VALUE appears, the
    // compiler will change it with 10 instead.
    //
    // instead of using hardcoded values, use
    // constants instead.
    const MAX_VALUE: u32 = 10;
    println!("MAX_VALUE: {}", MAX_VALUE);
    
    // SHADOWING
    let spaces = "   ";        // &str
    let spaces = spaces.len(); // usize
    println!("spaces: {}", spaces);
}
