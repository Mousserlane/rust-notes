# Enums & Pattern Matching

##### What is an Enum.

Enums allows us to define a type by enumerating its possible "variants". Where Structs give you a way of grouping together related fields & data, enums give you a way of saying a value is one of a possible set of values.


##### Enum methods

Like struct, enum can also have methods. The declaration & ruling is the same—it requires `&self` as the first variable.

```
...snip
impl IpType {
    fn get_subnet_mask(&self) -> IpType {
        match self {
            IpType::V4(_, _, _, _) => IpType::V4(255, 255, 255, 255),
            IpType::V6(_) => IpType::V6(String::from("::1")),
        }
    }
}

...snip inside fn main()

let home = IpType::V4(127, 0, 0, 0);
let subnetmask = home.get_subnet_mask();
```
