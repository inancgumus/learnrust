// OWNERSHIP
//
// MAIN IDEA:
//
// "SCOPE owns a VARIABLE owns a VALUE"
//
// + A value lives on memory.
// + It can move between variables.
// + A variable can be an owner of a value.
// + A variable comes in and goes out of a scope.
//
// RULES:
// + Each value has a single owner.
// + Ownership can move between variables.
// + When the owner goes out of scope, the value will be dropped.

fn main() {
    scope();
}

fn scope() {            // s isn't declared yet. not valid here.
    let s = "hi";       // s comes into scope here.

    println!("{}", s);  // work with s however you want.
    
}                       // s goes out of scope.
                        // you can no longer use it.
