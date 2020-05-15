use std::io;
use std::vec;

fn fibonacci(n: usize) -> Vec<u64> {
    if n == 1 {
        return vec![1];
    } else if n == 2 {
        return vec![1, 1];
    }

    let mut sequence: Vec<u64> = vec![1, 1];
    let mut i = 2;
    loop {
        sequence.push(sequence[i - 2] + sequence[i - 1]);
        i += 1;

        if i >= n {
            break;
        }
    }
    return sequence;
}

fn main() {
    println!("Please insert number between 1 and 93");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: usize = number.trim().parse().expect("Please type a number!");

    if number < 1 || number > 93 {
        println!("Invalid number!");
        return;
    }

    let sequence: Vec<u64> = fibonacci(number);
    let mut i = 0;
    loop {
        print!("{}", sequence[i]);
        i += 1;
        if i >= sequence.len() {
            break;
        }

        print!(",");
    }
    print!("\n");
}
