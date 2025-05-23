use std::io;

fn main() {
    // constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // variables and mutability
    let mut x = 5;
    println!("The value of x is: {x}.");
    x = 6;
    println!("The value of x is: {x}.");

    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y + 2;
        println!("inner scope y: {y}.");
    }

    println!("outer scope y: {y}.");

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}.");

    // other shadowing
    let spaces = "          ";
    let spaces = spaces.len();
    println!("spaces: {spaces}.");

    // data types
    if let Ok(guess) = "aa".parse::<u32>() {
        println!("guess: {guess}.");
    } else {
        println!("not a number");
    }

    // numeric operations
    let guess: i32 = "aa".parse().unwrap_or(0);
    println!("guess: {guess}.");

    let sum = 5 + 10;

    let difference = 10 - 5;

    let product = 10 * 5;

    let quotient = 10.1 / 5.1;

    let truncated = -10 / 5;

    let remainder = 11 % 5;

    println!("{} {} {} {} {} {}", sum, difference, product, quotient, truncated, remainder);

    // compound types
    // tuples
    let tup: (i32, f64, u8) = (500, 5.5, 5);

    let (x, y, z) = tup;

    println!("{:?}", tup);
    println!("{x} {y} {z}");
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // arrays
    let a = [1, 2, 3, 4, 5];
    let b = [3; 5];

    println!("{}, {}", a[1], b[2]);

    println!("Enter an array index.");

    let mut index = String::new();
    let mut i = "".to_string();

    io::stdin().read_line(& mut index).expect("Failed to read line.");
    let index: usize = index.trim().parse().expect("Not a number.");
    let element = a[index];
    println!("The value of index {index} is: {element}.")

}
