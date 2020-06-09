use std::io;
use std::io::Write;

fn caesar_cipher(input: &str, rot: u8) {
    let mut encrypted = String::with_capacity(input.len());
    const alphabet: &str = "abcdefghijklmopqrstuvwz";

}

fn main() {
    print!("Please insert a string: ");
    io::stdout().flush().unwrap();

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    input_string = String::from(input_string.trim());


}
