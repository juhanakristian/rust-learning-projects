use std::cmp;
use std::io;
use std::io::Write;

fn base64_encode(input: &[u8]) -> String {
    let base64_characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    // 010101 011111 111111
    let mut bits = 0;
    let mut output = String::new();
    loop {
        println!("bits {}", bits);
        let byte = bits / 8;
        println!("byte {}", byte);
        let bits_left = (byte + 1) * 8 - bits;
        if byte >= input.len() {
            break;
        }

        println!("bits_left {}", bits_left);
        let i = match bits_left.cmp(&6) {
            cmp::Ordering::Less => input[byte] << (6 - bits_left),
            cmp::Ordering::Greater => input[byte] >> (bits_left - 6),
            cmp::Ordering::Equal => input[byte],
        };

        let right_bits = 6 - bits_left;
        let index = match right_bits.cmp(&0) {
            cmp::Ordering::Equal => i & 0x3f,
            cmp::Ordering::Greater => (i | input[byte + 1] >> (8 - right_bits)) & 0x3f,
            cmp::Ordering::Less => i & 0x3f,
        };

        println!("{:#010b}", 0x3f);
        println!("{:#010b}", input[byte]);
        println!("{:#010b}", index);
        bits += 6;

        let character = base64_characters.chars().nth(index as usize).unwrap();
        output.insert(0, character);
    }

    return output;
}

#[test]
fn base64_encode_returns_correctly_encoded_value_when_no_padding() {
    let data: Vec<u8> = vec![116, 101, 115];
    let result = base64_encode(&data);
    assert_eq!(result, "dGVz");
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
