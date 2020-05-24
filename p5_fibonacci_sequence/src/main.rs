use std::io;
use std::vec;

fn fibonacci(n: usize) -> Vec<u64> {
    match n {
        0 => vec![],
        1 => vec![1],
        2 => vec![1, 1],
        _ => {
            let mut sequence: Vec<u64> = vec![1, 1];
            for i in 2..n {
                sequence.push(sequence[i - 2] + sequence[i - 1]);
            }
            sequence
        }
    }
}

fn main() {
    println!("Please insert number between 0 and 93");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: usize = number.trim().parse().expect("Please type a number!");

    if number > 93 {
        println!("Invalid number!");
        return;
    }

    let sequence: Vec<u64> = fibonacci(number);
    println!("{:?}", sequence);
}
