use std::cmp;
use std::io;
use std::io::Write;

fn base64_encode(input: &[u8]) -> String {
    let base64_characters = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
        "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9", "+", "/",
    ];

    let mut buf = 0;
    let mut bit_count = 0;
    let mut output = String::new();
    for &byte in input {
        for bit in 0..8 {
            buf = buf | (((byte as u8 >> (7 - bit)) & 0x01) << (5 - bit_count));
            bit_count += 1;

            if bit_count == 6 {
                output += base64_characters[buf as usize];

                bit_count = 0;
                buf = 0;
            }
        }
    }

    if bit_count > 0 {
        output += base64_characters[buf as usize];
    }

    // Add padding if needed. The output needs to be a multiple of 4.
    let pad = match output.len().cmp(&4) {
        cmp::Ordering::Less => 4 - output.len(),
        cmp::Ordering::Equal => 0,
        cmp::Ordering::Greater => output.len() % 4,
    };

    return output + &"=".repeat(pad);
}

#[test]
fn base64_encode_returns_correctly_encoded_value_when_no_padding() {
    assert_eq!(base64_encode("And".as_bytes()), "QW5k");
    assert_eq!(base64_encode("ÖöÖ".as_bytes()), "w5bDtsOW");
}

#[test]
fn base64_encode_returns_correctly_encoded_value_with_padding() {
    assert_eq!(base64_encode("t".as_bytes()), "dA==");
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
