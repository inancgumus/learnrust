// Variables have two type of semantics:
//
// 1. Move Semantics:
//
//    -> Variable's value moves to another variable
//       without copying.
//
//    -> Used for heap-allocated values like a String.
//
//
// 2. Copy Semantics:
//
//    -> Rust copies the variable's value, and uses
//       the new value for the new variable.
//
//    -> Used for scalar values like integers.
//
//    -> This kind of values usually live on stack.
//
// SEE: https://doc.rust-lang.org/std/marker/trait.Copy.html
fn main() {
    // =======================================
    // STACK MEMORY
    // =======================================

    // -> 5 can be allocated on the stack memory.
    // -> It's a simpler scalar value.
    // -> So Rust can copy it cheaply.
    let x = 5;

    // WHAT HAPPENS BELOW?
    //
    // 1. Rust COPIES x's value: 5.
    // 2. And BINDS the new value to the y variable.
    // 3. What the y contains is a copy.
    let y = x;
    // Here x and y have different memory locations.
    println!("x: {} y: {}", x, y);

    // =======================================
    // HEAP MEMORY
    // =======================================

    // WHAT IS A STRING?
    //
    // String::from("hi") looks like this:
    //
    // This part is usually       But this part should
    // allocated on the stack.    live on the heap.
    //
    // --------+---------         ----- + -----
    // name    | value            index | value
    // --------+---------         ----- + -----
    //  ptr    |  0x01  ------->    0       h
    //  len    |  2                 1       i
    //  cap    |  2               ----- + -----
    // --------+---------           ^
    //                              |
    //                              +-------+
    //                                      |
    // s1 contains the String value below.  |
    let s1 = String::from("hi"); //         |
    //                                      |
    // let s2 = s1;              //         | <-- READ THE CODE
    //                                      |
    // -> s2 is a new String value.         |
    // its ptr points to the same location  |
    // on the heap memory.                  |
    //                                      |
    // --------+---------                   |
    // name    | value                      |
    // --------+---------                   |
    //  ptr    |  0x01  --------------------+
    //  len    |  2             
    //  cap    |  2             
    // --------+---------         

    //
    // THE CODE BELOW WON'T WORK.
    //
    // println!("{} {}", s1, s2);

    //
    // WHY?
    //
    // -> Rust moves s1's value to s2.
    // -> s2 is the new OWNER of s1's value.
    // -> s1 is no longer valid.
    //    -> goes out of scope.
    //    -> rust claims its memory.

    //
    // WHAT WOULD WORK??
    //
    let s2 = s1.clone();           // expensive op
    println!("{} {}", s1, s2);
    // WHY?
    // -> s2 has a deep-copy of s1's value.
    // -> there are one more "hi" on the heap now.
    // -> and its owner is s2.

    //
    // BUT IT WAS WORKING WITH AN INTEGER LITERAL BEFORE?
    // (IN THE STACK MEMORY SECTION ABOVE)
    //
    // BUT WHY? TELL ME MORE:
    //
    // 1. Simple values like an integer doesn't need to be cloned.
    // 2. They can be copied by Rust automatically.
    // 3. It has a Copy treat.
    //    SEE: https://doc.rust-lang.org/std/marker/trait.Copy.html
    //
}