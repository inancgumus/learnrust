struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of a {}x{} rectangle is {} pixels.",
            rect.width,
            rect.height,
            // area(rect),  // this code is moving rect to area
            area(&rect), // but with this one, area is borrowing the rect
        );
        // if the area didn't borrow the rect,
        // the main would never use it again.
        // println!("{}", rect.width);
    }
    println!();
    // ----------------------------------------------------------------------
    {
        // Rust provides a number of traits for us to use with the derive
        // annotation that can add useful behavior to our custom types.

        // DEBUGGING STRUCTS
        #[derive(Debug)] // With this Debug trait, println can print
                         // an instance of the Rectangle struct.
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect = Rectangle {
            width: 25,
            height: 15,
        };

        println!("{:?}", rect); // You just need to use {:?} instead of {}.
        println!("{:#?}", rect); // Prettier output.
    }
}

// this function just immutably borrows a rectangle instance.
// it doesn't own the instance.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// this function signature is taking the ownership of a rectangle instance.
// fn area(rectangle: Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
