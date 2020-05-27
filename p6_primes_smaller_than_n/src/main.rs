use std::io;

fn primes_until(n: usize) -> Vec<u64> {
    let mut numbers: Vec<u64> = (2..=n as u64).map(u64::from).collect();

    let mut i: u64 = 0;
    loop {
        i = match numbers.iter().find(|&&x| x > i) {
            Some(x) => *x,
            None => break,
        };

        numbers.retain(|&x| x % i != 0 || x == i);
    }

    numbers
}

fn main() {
    println!("Please insert a number");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: usize = number.trim().parse().expect("Please type a number!");

    let primes = primes_until(number);
    println!("Primes smaller than {} are {:?}", number, primes);
}
