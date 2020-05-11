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
    let mut i = 0;
    loop {
        if i >= input_string.len() {
            break;
        }

        let character = input_string.pop();

        match character {
            Some(x) => input_string.insert(i, x),
            None => println!("String is empty!"),
        }

        i += 1;
    }

    println!("{}", input_string);
}
