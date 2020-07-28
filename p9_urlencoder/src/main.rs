use std::io;
use std::io::Write;

fn percent_encode(input: &str) -> String{
    let mut encoded = String::new();
    const reserved: &str = "!*'();:@&=+$,/?#[]";

    for i in 0..input.len() {
        let character = input.chars().nth(i).unwrap();
        match reserved.find(character) {
            Some(index) => {
                // Replace with percent encoded value
            }
            None => encoded.insert(i, character)
        }
    }

    return encoded;
}

fn main() {
    print!("Please insert a URL parameter: ");
    io::stdout().flush().unwrap();

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    input_string = String::from(input_string.trim());

    let encoded = percent_encode(&input_string);

    println!("{}", encoded);
}
