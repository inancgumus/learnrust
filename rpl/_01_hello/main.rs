/*
#1: defines a function named: `main`.

main function:
    + this is a special function.
    + it runs as the first code in an executable rust program.
*/
fn main() {
    /*    ^
          |
          between `{` and `}`

    this is where this function's body starts.

    =========================================================
    COMPILING & RUNNING THIS PROGRAM:
    =========================================================
    what you write in the function body will be executed,

      1. when you compile:

             rustc main.rs
        
      2. and run on xnix:

             ./main

      3. or run on windows:

             .\main.exe
    */

    println!("Hello, world!");
    //     ^                 ^
    /*     |                 +--------------------------------------------------------+
           |                                                                          |
    #3: calls a macro named: `println` that writes "Hello, world!" to the screen.     |
                                                                                      |
        + to call a macro   : println!("...")                                         |
        + to call a function: println("...")                                          |
        + you'll learn the differences later on.                                      |
                                                                                      |
                                                                                      |
    end your statements with this semicolon. <----------------------------------------+
    so the next one can begin.

    why?
    just like in C, and other languages with a similar syntax,
    semicolon here tells the compiler that this statement is over.
    */
}
