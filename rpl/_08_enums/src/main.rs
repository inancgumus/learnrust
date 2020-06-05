// An IP address can either be a version four or six, but not both.
//
// -> However, both versions are still fundamentally IP addresses.
// -> They should be treated as the same type.
//
// So, an IP address is a good candidate to be represented as an enum:
//
//      enum IpAddrKind {
//          V4,
//          V6,
//      }
//
// Usage:
//
//      V4 are V6 "variants" are "namespaced" (::) within the IpAddrKind enum.
//
//      let four = IpAddrKind::V4;
//      let six = IpAddrKind::V6;
//
// You can put it in a struct:
//
//      struct IpAddr {
//          kind: IpAddrKind, // <-
//          address: String,
//      }
//
// Usage:
//
//      let home = IpAddr {
//          kind: IpAddrKind::V4,
//          address: String::from("127.0.0.1"),
//      };
//
//      let loopback = IpAddr {
//          kind: IpAddrKind::V6,
//          address: String::from("::1"),
//      };
//
// However, there is a better way.
// -> Instead of using a struct,
// -> You can associate values with each variant of an enum.
enum IpAddr {
    // Enum variants can have associated values.
    V4(u8, u8, u8, u8), // -> represents an IPv4 address.
    V6(String),         // -> represents an IPv6 address.
}

// Each variant of the Message can store something else.
enum Message {
    Quit,                       // doesn't include anything
    Move { x: i32, y: i32 },    // includes an anonymous struct
    Write(String),              // includes a String
    ChangeColor(i32, i32, i32), // includes three i32 values
}
// Then you can easily use it with a fn like this:
//
//      fn processMessage(m: Message) {}
//
// But if we were to use a struct for each variant instead,
// then we wouldn't be able to define a single type to hold
// these all, and use with a fn easily.
//
//      struct QuitMessage;
//      struct MoveMessage {
//          x: i32,
//          y: i32,
//      }
//      struct WriteMessage(String);
//      struct ChangeColorMessage(i32, i32, i32);
//
//      fn processQuitMessage(m: QuitMessage) {}
//      fn processMoveMessage(m: MoveMessage) {}
//      fn processWriteMessage(m: WriteMessage) {}
//      fn processChangeColorMessage(m: ChangeColorMessage) {}
//

// You can also define methods on enums:
impl Message {
    fn process(&self) {
        // ...
    }
}

fn main() {
    // declaring enums with associated values.
    let _home = IpAddr::V4(127, 0, 0, 1); // has an [4]u8 tuple
    let _loopback = IpAddr::V6(String::from("::1")); // has a String

    // declaring and calling a method.
    let m = Message::Write(String::from("hi"));
    m.process();
}
