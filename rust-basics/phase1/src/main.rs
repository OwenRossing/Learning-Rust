fn main() {
    // Printing strings
    println!("Hello, world!");
    println!("Goooooon");
    println!("GOOOOOOON");
    println!("rizz");

    // Printing integers
    println!("{:b}", 42);
    println!("{}", -7);
    println!("{}", 0 + 1);

    // Statements vs Expressions
    let blue = {
        let a = 20;
        a + 22
    };
    println!("{}", blue);
}
