// Passing a value to a func is similar to assigning it to a variable.
// It will either MOVE or COPY the value.
fn main() {
    let mut s = String::from("hi");

    change_owner(s);                    // s's value (its pointer) moves 
                                        //       into the change_owner()
                                        //
                                        // s becomes invalid
                                        // -> rust has reclaimed its memory in
                                        //                  the change_owner()
    let n = 5;
    copy(n);                            // rust copies n (nothing moves)
    println!("{}", n);                  // -> so we can still use it

    // println!("{}", s);               // but we can't use s
                                        // -> it was moved to the change_owner()

    s = give_owner();                   // -> s owns give_owner's s's value now
                                        // -> s is valid now
    let s2 = String::from("hello");
    let s3 = change_and_give_owner(s2); // -> s2 looses ownership to the func
                                        // -> and the func gives it back to s3

    // s2 is no longer valid:
    // println!("s: {} s2: {} s3: {}", s, s2, s3);
    println!("s: {} s3: {}", s, s3);
    
}   // out of scope: s and s3 are dropped.

fn change_owner(s: String) {            // main's s's new owner is this func
    println!("{}", s);
}                                       // out of scope: s is dropped
                                        // rust reclaims its memory

fn copy(i: i32) {                       // receives a copy of n as i
    println!("{}", i);
}

fn give_owner() -> String {
    let s = String::from("hey");        // s is the owner of String("hey")
    
    s                                   // give_owner loses the ownership to
                                        // the calling func.
                                        // -> s's value will live in the calling func.
}


fn change_and_give_owner(s: String) -> String {  // becomes the owner of s
    s                                            // loses the ownership of s to
                                                 // the calling func.
}