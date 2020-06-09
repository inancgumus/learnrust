# FAQ

* [Why is it discouraged to accept a reference to a String (&String), Vec (&Vec), or Box (&Box) as a function argument?](https://stackoverflow.com/questions/40006219/why-is-it-discouraged-to-accept-a-reference-to-a-string-string-vec-vec-o)
  * To allow more generic code.

* [What are the differences between `String` and `str`?](https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str)
  * `String` is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.
  * `str` is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory. Since the size is unknown, one can only handle it behind a pointer. This means that str most commonly2 appears as &str: a reference to some UTF-8 data, normally called a "string slice" or just a "slice".
  * Use `String` if you need owned string data (_like passing strings to other threads, or building them at runtime_), and use `&str` if you only need a view of a string.

* [Why are explicit lifetimes needed in Rust?](https://stackoverflow.com/questions/31609137/why-are-explicit-lifetimes-needed-in-rust)
  * The main reason is that while the compiler can see what your code does, it doesn't know what your intent was.

* [What is the difference between iter and into_iter?](https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter)
  * `iter()` iterates over the items by _borrowing_.
  * `into_iter()` iterates over the items, _moving_ them into the new scope (_takes ownership_).
  * `iter_mut()` iterates over the items, giving a _mutable reference_ to each item.