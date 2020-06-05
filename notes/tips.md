# Tips


## General

1. Create SMALL rust binaries, see: [min-sized-rust](https://github.com/johnthagen/min-sized-rust).
2. Compile simple programs w/`rustc`, but larger programs w/`cargo`.
3. Format your code automatically: `rustfmt`.


## Cargo

1. Creates a new crate (it's like `go mod init`):
   `cargo new <name>`

   Without a git repository:
   `cargo new <name> --vcs none`

2. Build the crate (it's like `go create`):
   `cargo build`
   `cargo b`

3. Run the crate (it's like `go run`):
   `cargo run`
   `cargo r`

   1. Silently run:
   `cargo r -q`

4. Check whether your program can compile:
   `cargo check`
   `cargo c`

   Rustaceans use this command to periodically check their programs. After working on their program, and when they're sure, they build your program using `cargo build --release`.

   This command produces a faster program, but it'll take a longer time to compile.

   This is also the command you want to use when you benchmark your code.

5. Cargo caches. If you want to start on a clean slate, run:
   `cargo clean`

   Why? Sometimes, `cargo check` returns a warning. However, when you run it the second time, it doesn't display the warning. In that case, you can run `cargo clean`, and then run `cargo check` again. This way, you'll be able to see the error message again.

6. You can see the documentation of every crate that your program/package depends on.
   `cargo doc --open`

7. Create a library package.
   `cargo new --lib <name>`
   _This command will create a package with a test._

8. Work on someone else's project:
   ```bash
   git clone url/project
   cd project
   cargo build
   ```

## Memory Leaks

1. Use [Valgrind](https://www.valgrind.org/info/) to check for memory errors.