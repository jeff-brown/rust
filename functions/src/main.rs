fn main() {
    println!("Hello, world!");
    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = five_plus_one(x);
    println!("The value of x plus one is: {}", x);
}

fn another_function(x: i32, u: char) {
    println!("The value of x is {x}{u}.");
}

fn five() -> i32 {
    5
}

fn five_plus_one(x: i32) -> i32 {
    x + 1
}
