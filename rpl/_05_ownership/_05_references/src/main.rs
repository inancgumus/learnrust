//
// WHAT IS A REFERENCE?
//
// It's a value that refers to another value
// without taking its ownership.
//
// Represented with a leading ampersand &.
//
//      &s          let s = String::from("hi")       "hi"
//
// name | value           name | value          index | value
// ---- + ------          ---- + ------         ----- + -----
// ptr  |  0x5  ------->  ptr  |  0x01  ------->  0   |   h
//                        len  |  2               1   |   i
//                        cap  |  2             ----- + -----
//                        -----+-------
//
fn main() {
    let s = String::from("hi");

    let l = strlen(&s);              // strlen BORROWS s.
                                     // strlen doesn't own it.
                                     // main is the OWNER.
                                     // strlen is the BORROWER.

    println!("len({}) = {}", s, l);  // that's why we can still use s here.
    
    // 

    let mut cs = s;                  // cs is the owner of s now.

    change(&mut cs);                 // send a mutable ref of cs with change()
                                     // -> so change() can change it.

    println!("{}", cs);              // we can still use cs because
                                     // we're the owner of it.
    println!("{:?}", cs);

    // ========================================================================
    // MULTIPLE MUTABLE REFERENCES
    //
    // let mut s = String::from("hey");
    //
    // IN THE SAME SCOPE:
    //
    // -> There can be only a single mutable borrower (reference).
    // {
    //     let mutRefToS = &mut s;
    //
    // }
    // mutRefToS goes out of scope, Rust drops it.
    // That's why you can make a new reference to it here.
    //
    // let mutRef2ToS = &mut s;
    //
    // -> There can be multiple non-mutable borrowers.
    // let rs1 = &s;     // immutable borrower
    // let rs2 = &s;     // immutable borrower
    //
    // -> There can't be a mutable borrower if there are immutable borrowers.
    // let rs3 = &mut s; // mutable borrower
    // println!("{} {} {}", rs1, rs2, rs3);
    //
    // ========================================================================
        
}   // the main is the owner of s, and cs.
    // they go out of scope and but Rust drops them.

fn strlen(s: &String) -> usize {     // s is a reference to a String
    s.len()
}   // s goes out of scope but nothing happens.
    // because strlen isn't the owner of s,
    // the main() is.

/*
this won't work.
s is not a mutable reference.

fn change(s: &String) {
    s.push_str(" there!");
}
*/
fn change(s: &mut String) {
    s.push_str(" there!");
}