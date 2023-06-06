fn main() {
    // let number = 7;
    // if number > 5 {
    //     println!("Condition === truthy");
    // } else {
    //     println!("Condition === falsy");
    // }
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");
}
