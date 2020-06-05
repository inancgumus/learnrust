# Shocks

Weird Rust stuff documented.

1. **There is no null in Rust!**

   Instead, there is `None`, and there is:

   ```rust
   enum Option<T>{
      Some(T),
      None
   }
   ```

2. **String `AsRef<Path>`**

   Why the heck `String` has a responsibility for `Path`...

3. **Why Rust doesn't allow marking only some struct fields mutable?**

   Each field is in a different memory location. However, the struct
   is a type. So, when using it, we can't know which part of the
   struct's instance that the other code is going to use.

4. **I couldn't grasp how to run multiple files together from get-go**

    src/main.rs: 

    ```rust
       mod vectors;
       fn main() {
           vectors::run_vectors();
       }
    ```

    src/vectors.rs: 
    
    ```rust
       pub fn run_vectors() {
           println!("vectors!");
       }
    ```

    👉 [See this for the best explanation](https://learning-rust.github.io/docs/d3.modules.html).

5. **String concatenation uses the buffer of the origin string**

   This is a good thing as in the op below, Rust will copy str2 to
   str1's buffer.

   ```rust
   let str3 = str1 + &str2;
   ```

6. **People can add their own hashers to stdlib's HashMap implementation**

   Check out them [here](https://crates.io/search?q=hasher).