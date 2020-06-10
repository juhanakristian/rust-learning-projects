use std::io;
use std::io::Write;

fn caesar_cipher(input: &str, rot: usize) -> String{
    let mut encrypted = String::with_capacity(input.len());
    const ALPHABET: &str = "abcdefghijklmopqrstuvwz";

    for i in 0..input.len() {
        let character = input.chars().nth(i).unwrap();
        match ALPHABET.find(character) {
            Some(index) => {
                let mut next_index = index + rot;
                if next_index > ALPHABET.len() {
                    next_index = next_index - ALPHABET.len();
                }
                match ALPHABET.chars().nth(next_index) {
                    Some(encrypted_char) => encrypted.insert(i, encrypted_char),
                    None => encrypted.insert(i, character)
                }
            }
            None => encrypted.insert(i, character)
        }
    }

    return encrypted;
}

fn main() {
    print!("Please insert a string: ");
    io::stdout().flush().unwrap();

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    input_string = String::from(input_string.trim());

    let encrypted = caesar_cipher(&input_string, 5);

    println!("{}", encrypted);
}
