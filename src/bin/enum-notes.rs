// Enums allows us to define a type by enumerating its possible "variants".
// Where Structs give you a way of grouping together related fields & data, enums give you a
// way of saying a value is one of a possible set of values.

#[derive(Debug)]
enum IpType {
    // Defining an enum with type
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enum can also have methods. Like in struct, the first variable must be self
impl IpType {
    fn get_subnet_mask(&self) -> IpType {
        // match is used to determine the right return value based on which type of IpType is
        // making the call.
        match self {
            // It requires pattern to make the syntax correct. If the variable in the pattern is unused, then
            // use "_" to omit it.
            IpType::V4(_, _, _, _) => IpType::V4(255, 255, 255, 255),
            IpType::V6(_) => IpType::V6(String::from("::1")),
        }
    }
}

fn main() {
    let home = IpType::V4(127, 0, 0, 0);
    let loopback = IpType::V6(String::from("::1"));

    let subnetmask = home.get_subnet_mask();
    let subnetmask_v6 = loopback.get_subnet_mask();

    println!("The subnet mask is {:?}", subnetmask);
    println!("V6 loopback is {:?}", loopback);
    println!("V6 subnet mask is {:?}", subnetmask_v6)
}
