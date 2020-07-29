use std::io;
use std::io::Write;

pub struct Reserved {
    character: char,
    encoded: &'static str
}

impl PartialEq<char> for Reserved {
    fn eq(&self, other: &char) -> bool {
        return self.character == *other;
    }
}

fn percent_encode(input: &str) -> String{
    let mut encoded = String::new();
    // const reserved: &str = "!*'();:@&=+$,/?#[]";
    const unreserved: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";

    let reserved = vec![
        Reserved {character: '!', encoded: "21"},
        Reserved {character: '*', encoded: "21"},
        Reserved {character: '\'', encoded: "21"},
        Reserved {character: '(', encoded: "21"},
        Reserved {character: ')', encoded: "21"},
        Reserved {character: ';', encoded: "21"},
        Reserved {character: ':', encoded: "21"},
        Reserved {character: '@', encoded: "21"},
        Reserved {character: '&', encoded: "21"},
        Reserved {character: '=', encoded: "21"},
        Reserved {character: '+', encoded: "21"},
        Reserved {character: '$', encoded: "21"},
        Reserved {character: ',', encoded: "21"},
        Reserved {character: '/', encoded: "21"},
        Reserved {character: '?', encoded: "21"},
        Reserved {character: '#', encoded: "21"},
        Reserved {character: '[', encoded: "21"},
        Reserved {character: ']', encoded: "21"},
    ];

    for i in 0..input.len() {
        let character = input.chars().nth(i).unwrap();
        // Handle allowed characters
        if unreserved.contains(character) {
            encoded.insert(i, character)
        }

        
        let mut iter = reserved.iter();
        match iter.find(|x| **x == character) {
            Some(index) => {
                // Replace with percent encoded value
            }
            None => {
                encoded.insert(i, character)
            }
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
