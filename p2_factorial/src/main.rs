use std::io;

fn factorial(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    return n * factorial(n - 1);
}

fn main() {
    println!("Please insert number");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Please type a number!");

    println!("Factorial of {} is {}", number, factorial(number));
}
