fn main() {
    // The default types for integer is i32(signed 32 bit)
    // which can store the maximum number of (2^32-1) - 1 = 2.147.483.647
    // so, the code below won't compile;
    // If we need bigger number then we need to increase the bit size.
    // let some_integer: i32 = 2147483649;
    // println!("some integer is: {some_integer}");

    // Declaring a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 253);
    // Desctrucuring a tuple || getting individual values out of a tuple.
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    // we can also access tuple value like so:
    let first_tuple = tup.0;
    println!("the tuple x is: {first_tuple}");

    // declaring an array.
    // Mind you, array in rust have a fixed length
    // If we need a dynamic collection type, we should use Vector instead.
    let _an_array = [1, 2, 3, 4, 5];

    // we can explicitly define the array type & its length like so:
    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // you can also do this
    let _a = [3; 5]; // This will contain 5 elements with value of 3 aka [3,3,3,3,3];
    let get_five = five();
    println!("{get_five}")
}

// This is a valid function that returns a value. You need to specify the return type of the function
fn five() -> i32 {
    // If we add a semicolon after 5, we will get an error stating that the function has no return value.
    // This is because by adding semicolon, we're transforming the function body from an expression
    // to a statement. See the Book page 46 to see the difference between them.
    5
}
