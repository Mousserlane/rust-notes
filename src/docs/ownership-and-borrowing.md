# Ownership & Borrowing

Rules concerning ownership:

1. Each value in Rust has an owner.

2. There can only be one owner at a time
3. When owner goes out of scope, the value will be dropped.


The code snippet below will try to explain point number 3:

```
fn scope_of_s() {
    // Consider the variable below
    let s = "Hello"; // s is valid from this point onward.
    println!("{s}");
} // this scope is over & s is no longer valid.
```
