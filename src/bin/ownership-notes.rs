fn main() {
    // This program will walk through the concept of Ownership, borrowing, & string slices.
    // This is a really huge topic to cover so please revisit some of the concept in The Book,
    // especially about Stack & Heap.

    // Rules concerning ownership:
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time
    // 3. When owner goes out of scope, the value will be dropped.

    // Read the function note
    scope_of_s();

    // When a variable is out of scope, Rust calls a special function for us. It is called
    // `drop`. It is called automatically at the closing bracket

    // The function below attempts to explain the difference between value copy & pointer copy
    value_or_pointer_copy();
}

fn value_or_pointer_copy() {
    // The statement below is a simple example of copy by value. This is achieveable with integers
    // because they are simple values with a known, fixed size. These two 5 values are pushed onto
    // the stack.
    let x = 5;
    let y = x;

    // However, with strings, things are a bit more complicated.
    let s1 = String::from("Hello");
    let s2 = s1;
    // A string is made up of three parts:
    // 1. A pointer to the memory that holds the contents of the string: (See The book for diagram—Figure 4-1)
    // 2. A length
    // 3. A capcity
    // Since it will be very expensive to copy string data, what happen is that Rust will copy the String data & Not
    // the actual data. Then, Rust will use the pointer & use it as reference to the data. See
    // Figure 4-2 in The book for clearer explanation.

    // Note that after :30—where the reassignment happen—Rust no longer considers s1 as valid.
    // uncomment the code below for proof
    // println!("{s1}");

    // This is because a potential `double free` error where both variable will try to free the same memory.
    // Freeing memory twice can lead to memory corruption, which can lead to security vulnerabilities.
    println!("{s2}, {y}"); // This line is just to avoid unused variable

    //Because rust also invalidates the first variable, instead of being called a shallow copy, it is known as a `move`.
    // We would say that s1 was `moved` into s2.

    // also NOTE that Rust will never automatically create "deep" copies of your data. Therefore, any
    // automatic copying can be assued to be inexpensive in terms of runtime performance.
}

fn scope_of_s() {
    // Consider the variable below
    let s = "Hello"; // s is valid from this point onward.
    println!("{s}");
} // this scope is over & s is no longer valid.
