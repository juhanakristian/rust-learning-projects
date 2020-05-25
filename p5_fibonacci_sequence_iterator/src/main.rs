use std::io;

// Solution by Isaac Azuelos
// https://twitter.com/isaacazuelos/status/1264652368127246336/photo/1
struct Fib {
    a: u64,
    b: u64
}

impl Fib {
    fn new() -> Fib {
        Fib {
            a: 1, b: 0
        }
    }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let Fib {a, b} = *self;

        self.a = a + b;
        self.b = a;
        Some(b)
    }
}

fn fibonacci(n: usize) -> Vec<u64> {
    Fib::new().take(n).collect()
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
