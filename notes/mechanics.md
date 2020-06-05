# Mechanics

## Error Handling

* Rust has a built in generic enum called Result.
* It allows us to return a value that has the possibility of failing.
* It is the idiomatic way in which the language does error handling.

```rust
enum Result<T,Q>{
    Ok(T),
    Err(Q)
}
```

### Example:

```rust
fn might_fail(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))   
    }
}
```

#### Verbose Way:

```rust
fn main() {
    let result = do_something_that_might_fail(12);
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}",e),
    }
}
```

#### Concise Way:

```rust
fn main() -> Result<(), String> {
    let v = do_something_that_might_fail(42)?;
    println!("found {}", v);
    Ok(())
}
```

#### Bad Way:

**Don't use `unwrap()`.**

```rust
fn main() -> Result<(), String> {
    // concise but assumptive and gets ugly fast
    let v = do_something_that_might_fail(42)?;
    println!("found {}", v);
    
    // this will panic!
    let v = do_something_that_might_fail(1)?;
    println!("found {}", v);
    
    Ok(())
}
```

---