fn main() {
    let number = 7
    
    if number % 2 == 0 {
        println!("Number is even")
    } else {
        println!("Number is odd");
    }

    // Match expression
    match number {
        0 => println("It is zero"),
        1 | 2 => println!("It is one or 2"),
        _ => println!("It is something else"),
    }
    
    for i in 1..=5 {
    println!("{}", i);
    }
}
