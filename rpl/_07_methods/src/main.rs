#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are used for organization and readability purposes.
//
// You can declare methods with Implementations blocks like the one below.
//
// Rust wants you to put everything related to an instance of a type into
// a single `impl` block like this. So that, no one would've to search for
// the stuff they contain.
impl Rectangle {
    // add the area method to the Rectangle.
    // this method accepts a ref to a Rectangle instance.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // this method also accepts another Rectangle ref.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // ASSOCIATED FUNCS (AKA STATIC METHODS)
    // An associated func belongs to its type, instead of one of its instances.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    let r2 = Rectangle {
        width: 10,
        height: 40,
    };
    let r3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("r1 => {:?}", r1);
    println!("r2 => {:?}", r2);
    println!("r3 => {:?}", r3);
    println!(
        "The area of r1 is {} pixels.",
        r1.area(), // calls the area method by sending &r1 to
                   // the area method as the first argument.,
    );
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r1 hold r3? {}", r1.can_hold(&r3));

    // Automatic Referencing:
    //      (&r1).area();
    // Above is equal to:
    //      r1.area();
    // Rust can also detect whether the instance variable is mutable or not:
    //      (&mut r1).area();

    // ------------------------------------------------------------------------

    // ASSOCIATED FUNCS
    //
    // -> :: is the namespace syntax.
    // -> An associated func is namespaced within a type.
    // -> Here the square fn is namespaced within the Rectangle type.

    let sq = Rectangle::square(10);
    println!("sq => {:?}, area => {}", sq, sq.area());
}
