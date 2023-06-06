// This program will attempt to explain struct by example;

// Struct is a custom data type that allows you to package together
// and name multiple related values that makes up a group

// We can define struct by using the keyword struct and its name.
// Inside of the struct, we'll define the properties and its type e.g.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs: Have the added meaning the struct name provides but don't have
// names associated with their fields & just the type e.g.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // To use a struct, we create an "instance" of that struct by specifying
    // concrete values for the properties. e.g.
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("some_email@someprovider.com"),
    //     sign_in_count: 1,
    // };
    // Note that the value assignment does not have to be in particular order.
    // The order can be arbitrary but it must adhere to the types & property names
    // declared by the Struct.

    let mut user1 = user_factory(
        String::from("some_email@someprovider.com"),
        String::from("someusername123"),
    );

    // To get a specific value, we use the dot notation. For example:
    user1.email = String::from("new_email@someprovider.com"); // In order to mutate struct values, the instance must be mutable
                                                              // we are not allowed to mark only certain fields as mutable.
                                                              // it is often useful to create a new instance of a struct that includes
                                                              // most of the values from another instance, but changes some. We can do this
                                                              // by using the struct update syntax i.e.
    let user2 = User {
        email: String::from("new_email@someprovider.com"),
        ..user1 // Kinda similar to the spread syntax in JS & it did the same thing.
                // One thing to note is that the update syntax(..) must come last to specify
                // that any remaining fields should get their values from corresponding fields
                // in user1
    };
    // Note that we can no longer use user1 because it has been "Moved"(note the '=') into user2

    // Here's something interesting: If we only update 1 field that has drop trait i.e String,
    // then user1 can no longer be used. However, if we update both "email" & "username" & thus only
    // used the active and sign_in_count values from user1 then user1 would still be valid after
    // creating user2. This is because the aforementioned variables implements the Copy trait,
    // so the behavior in page 68 of The Book would apply. But it still don't makes sense to me...
    // We are updating the value that implements Drop trait, yet if we update both, user1 is still valid because
    // the rest of the properties that implements Copy trait is not updated...

    //================================
    // implementation of Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let next = Point(10, 20, 0);
    // Note that the black and origin values are different type because they're instances of
    // different tuple structs. Each struct you define is its own type, even though the fields might
    // have the same type. Therefore, type Color cannot take a Point as an argument even though
    // both types are made of three i32 values.
}

fn user_factory(email: String, username: String) -> User {
    // As with any expression, we can construct a new instance of the struct as the last expression in the function body
    // to implicitly returns a User instance with given email & username.
    User {
        active: true,
        email, // We can also use shorthand, provided that the parameter name & property name are the same—like in JS
        username,
        sign_in_count: 1,
    }
}
