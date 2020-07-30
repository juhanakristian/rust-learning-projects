use std::io;
use std::io::Write;

fn utf8_to_bytes_string(buffer: [u8; 4]) -> String {
    return buffer
            .iter()
            .filter(|&x| *x != 0)
            .map(|v| format!("%{:X}", v))
            .collect::<Vec<String>>()
            .join("");
}

fn encode(character: char) -> String {
    let mut buffer: [u8; 4] = [0; 4];
    let str_character = character.encode_utf8(&mut buffer);

    const UNRESERVED: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";
    if UNRESERVED.contains(character) {
        return str_character.to_owned();
    }

    return utf8_to_bytes_string(buffer);
}

fn percent_encode(input: &str) -> String{
    return input.chars().map(|c| encode(c)).collect();
}

#[test]
fn it_encodes_reserved_values_correctly() {
    assert_eq!("%21", percent_encode("!"));
    assert_eq!("%23", percent_encode("#"));
    assert_eq!("%24", percent_encode("$"));
    assert_eq!("%25", percent_encode("%"));
    assert_eq!("%26", percent_encode("&"));
    assert_eq!("%27", percent_encode("'"));
    assert_eq!("%28", percent_encode("("));
    assert_eq!("%29", percent_encode(")"));
    assert_eq!("%2A", percent_encode("*"));
    assert_eq!("%2B", percent_encode("+"));
    assert_eq!("%2C", percent_encode(","));
    assert_eq!("%2F", percent_encode("/"));
    assert_eq!("%3A", percent_encode(":"));
    assert_eq!("%3B", percent_encode(";"));
    assert_eq!("%3D", percent_encode("="));
    assert_eq!("%3F", percent_encode("?"));
    assert_eq!("%40", percent_encode("@"));
    assert_eq!("%5B", percent_encode("["));
    assert_eq!("%5D", percent_encode("]"));
}

#[test]
fn it_encodes_unicode_values_correctly() {
    assert_eq!("%E2%82%AC", percent_encode("€"));
    assert_eq!("%C3%96%C3%B6", percent_encode("Öö"));
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
