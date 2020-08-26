use std::cmp;
use std::io;
use std::io::Write;
use std::iter;

fn base64_encode(input: &[u8]) -> String {
    let base64_characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut bits = 0;
    let mut output = String::new();
    loop {
        let byte = bits / 8;
        let bits_left = (byte + 1) * 8 - bits;
        if byte >= input.len() {
            break;
        }

        let index = match bits_left.cmp(&6) {
            cmp::Ordering::Less => {
                if byte + 1 < input.len() {
                    let l = input[byte] << (6 - bits_left);
                    let r = input[byte + 1] >> (8 - (6 - bits_left));
                    (l | r) & 0x3f
                } else {
                    (input[byte] << (6 - bits_left)) & 0x3f
                }
            } // We need more bits!
            cmp::Ordering::Greater => (input[byte] >> (bits_left - 6)) & 0x3f,
            cmp::Ordering::Equal => input[byte] & 0x3f,
        };

        bits += 6;

        output.push(base64_characters.chars().nth(index as usize).unwrap());
    }

    // Add padding if needed. The output needs to be a multiple of 4.
    let mod4 = output.len() % 4;
    let padding = "=".repeat(mod4);
    output += &padding;

    return output;
}

#[test]
fn base64_encode_returns_correctly_encoded_value_when_no_padding() {
    assert_eq!(base64_encode("And".as_bytes()), "QW5k");
}

#[test]
fn base64_encode_returns_correctly_encoded_value_with_padding() {
    assert_eq!(base64_encode("TEST".as_bytes()), "AVRFU1Q=");
}

fn main() {
    print!("Please insert text to encode: ");
    io::stdout().flush().unwrap();

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    input_string = String::from(input_string.trim());

    let encoded = base64_encode(input_string.as_bytes());

    println!("{}", encoded);
}
