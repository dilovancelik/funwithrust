use std::io;

fn main() {
    println!("What which fibonacci number do you want");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Faild to read line");

    let number: u32 = number.trim().parse()
        .expect("You did not input a positiv number");

    let fibonacci_number = fibonacci(number);

    println!("The Fibonacci at index {0} is: {1}", number, fibonacci_number);
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0   =>  panic!("zero is not a proper argument!"),
        1|2 =>  1,
        3   =>  2,
        _   => fibonacci(n - 1) + fibonacci(n - 2)
    }
}