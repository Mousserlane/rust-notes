fn main() {
    let mut count = 1;
    'outer: loop {
        println!("London bridge is falling down");
        let mut falling_down = 1;
        loop {
            // Still has a bug that prints falling down before my sweet lady
            println!("Falling down {falling_down}");
            if falling_down == 2 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            falling_down += 1;
        }
        count += 1;
    }
    println!("My sweet lady")
}
