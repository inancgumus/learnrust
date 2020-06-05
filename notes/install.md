# Install Rust

## Installing manually

* I recommend you to use `rustup` (instructions below).
* If you want to do it the hard way—manually—see [this](https://www.rust-lang.org/tools/install/)


## rustup
* Use this one to install any Rust version.
  `curl https://sh.rustup.rs -sSf | sh`

* You can even uninstall Rust:
  `rustup self uninstall`

* Let's see the Rust documentation offline:
  `rustup doc`

* You can update to a newer Rust version (*if you'd like*):
  `rustup update`


## Solutions to possible problems

### VSCode integration problems
* **If you see these errors:**
  * Couldn't start client Rust Language Server.
  * Rustup not available.
* **Add this to your config:**
      ```json
      "rust-client.rustupPath": "~/.cargo/bin/rustup",
      ```

### Toolchain running problems
* **If you see the error when running `cargo` on OS X:**
  * `"Library not loaded: /usr/local/opt/openssl/lib/libssl.1.0.0.dylib" cargo`
* **Change your OpenSSL version:**
      ```bash
      brew switch openssl 1.0.2r
      ```