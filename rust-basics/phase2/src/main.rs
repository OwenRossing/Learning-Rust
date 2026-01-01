fn main() {
    // Immutable vs Mutable
    let x = 5; // Immutable variable
    // x = 6; // This line would cause a compile-time error
    let mut y = 10; // Mutable variable
    y = y - 15;
    y = y + x + 3;
    println!("{}", y);

    // Shadowing
    let z = 8;
    let z = z + 2; // Shadows previous z
    {
        let z = z * 2; // Shadows previous z within this block
        println!("{}", z); // Prints 20
    }
    println!("{}", z); // Prints 10

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // There were more that i forgot about
}
