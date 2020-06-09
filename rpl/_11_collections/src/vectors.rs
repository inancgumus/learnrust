// ⭐️ A vector is stored on the heap memory.

pub fn run() {
    //
    // let's create a vector of 32-bit ints:
    //                __________/
    //               /
    let mut v: Vec<i32> = Vec::new();
    //
    // let's create the same vector w/type-inferring:
    //
    //      let v = vec![1, 2, 3];
    //
    // let's create another i32 vector that looks like this: `[1, 1, 1]`.
    //
    //      let v = vec![1; 3];
    //

    // let's update the vector
    v.push(1);
    v.push(2);
    v.push(3);

    println!("\nreading a vector:");
    println!("first : {}", &v[0]);
    println!("second: {}", v.get(1).unwrap()); // use this with `match`.

    //
    // Borrowing and Ownership:
    //
    // let first = &v[0];              // immutable borrow
    // v.push(4);                      // mutable borrow   -> ERROR
    // println!("first: {}", first);   // immutable borrow
    //
    // A vector can move its elements (behind the scenes).
    // That's why even borrowing the first element, then pushing
    // another one at the end can create an error here.
    //

    println!("\nwalking over a vector:");
    for i in &mut v {
        // v is a Vec<i32>
        // i is a `&mut i32`, and `*i` is an i32.
        // Here, we're writing to the memory cell where i lives by dereferencing.
        *i *= 2; // write to the vector element

        println!("{}", i); // read the vector element
    }
    // To walk over immutably, use:
    // for i in &v {..}
}
// Rust will drop `v` here, along with its elements.
