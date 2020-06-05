// You can use a struct type (also an enum type) represent a concept, and organize similar data under a single umbrella.

// User is a struct type
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // root is an instance of the User struct.
    //
    // to change the root instance, its entire instance
    // should be declared as mutable.
    let mut root = User {
        username: String::from("guest"),
        email: String::from("root@universe.com"),
        sign_in_count: 5,
        active: false,
    };

    // change the username field
    root.username = String::from("root");

    println!("{}'s email: {}", root.username, root.email);
    // ----------------------------------------------------------

    let guest = build_user(String::from("guest"), String::from("guest@srv.com"));
    let active = if guest.active { "active" } else { "not active" };
    println!("{}'s email: {}", guest.username, guest.email);
    println!("\t{} and signed on {} times.", active, guest.sign_in_count);
    // ----------------------------------------------------------
    let jack = User {
        username: String::from("jack"),
        email: String::from("jack@secret.org"),
        // CUMBERSOME:
        // active: root.active,
        // sign_in_count: root.sign_in_count,
        //
        // STRUCT UPDATE SYNTAX:
        ..root
    };
    let active = if guest.active { "active" } else { "not active" };
    println!("{}'s email: {}", jack.username, jack.email);
    println!("\t{} and signed on {} times.", active, jack.sign_in_count);

    // ----------------------------------------------------------
    // TUPLE STRUCTS

    // Example: struct Point(i32, i32);
    // -> Have a type name: Point.
    // -> Have fields     : (i32, i32).
    // -> Fields don't have names.
    //
    // When to use?
    // -> To give the tuple a name to make it different from other types.
    // -> When naming every other field a name is verbose and reduntant.

    struct Color(i32, i32, i32); // (Red, Green, Blue)
    struct Point(i32, i32, i32); // (X,   Y,     Z)

    let mut _black = Color(0, 0, 0);
    let mut _origin = Point(0, 0, 0);

    // Each struct you define is its own type,
    // even though the fields within the struct
    // have the same types.
    //
    // _black = _origin     // MISMATCHED TYPES:
    // _black is a Color
    // _origin is a Point

    // You can access tuple struct fields using their indexes.
    println!("R: {}, G: {}, B: {}", _black.0, _black.1, _black.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        // CUMBERSOME:
        // email: email,
        // username: username,
        //
        // FIELD-INIT SHORTHAND:
        email,
        username,

        sign_in_count: 1,
        active: true,
    }
}
