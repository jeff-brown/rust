fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    
    let mut count = 0;
    'outer_loop: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        
        'inner_loop: loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break 'inner_loop;
            }
            if count == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
    
    let mut number = 10;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    };
    println!("LIFTOFF!!!");
    
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    };
    
    for element in a.iter() {
        println!("the value is: {}", element);
    };
    
    for number in (1..10).rev() {
        println!("{}!", number);
    };
    println!("LIFTOFF!!!");
}
