struct Rectangle {
    width: u32,
    height: u32,
}

// A method is similar to function. It is declared with the same keyword `fn methodName`, can have parameters & return values.
// But unlike functions, methods are defined within the context of a struct & their first parameter will always `self` which
// represents the instance of the struct the method is being called on.

// this is how one define a method
impl Rectangle {
    // A method of area is now defined in the context of rectangle.
    fn area(&self) -> u32 {
        // The value of self contains properties that are defined inside of the struct.
        // &self is a short for self: &Self.
        // Methods can take ownership of self, borrow self immutably, or borrow self mutably.
        self.width * self.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    // Method syntax goes AFTER an instance.
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    ); // calling the method
}
