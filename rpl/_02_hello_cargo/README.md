# First Cargo Program

* You can create a similar directory by typing:

  ```bash
  cargo new _02_hello_cargo
  ```

* This command creates a [Cargo.toml](Cargo.toml) file, click on the link at the left to learn more about it.

* Then, start writing code in [src/main.rs](src/main.rs).

* Cargo uses Cargo.lock file to fix the crate versions (aka: _package in other langs_). It is idiomatic to save this file to [git](https://git-scm.com/) for executable programs. Otherwise, if you're building a rust library, then add this to your `.gitignore`. To learn why, [read this](https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries).

* See [this](/notes/tips.md#cargo) to understand how to run a program using Cargo.

---

👉 You can also read more about Cargo [here](https://doc.rust-lang.org/cargo/).