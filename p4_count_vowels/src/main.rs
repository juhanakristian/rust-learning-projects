use std::io;
use std::io::Write;

fn main() {
    print!("Please insert a string: ");
    io::stdout().flush().unwrap();

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    input_string = String::from(input_string.trim());

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;
    for c in input_string.chars() {
        if vowels.contains(&c) {
            count += 1;
        }
    }

    println!("Vowel count {}", count);
}
