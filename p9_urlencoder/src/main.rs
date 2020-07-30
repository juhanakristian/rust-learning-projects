use std::io;
use std::io::Write;

fn percent_encode(input: &str) -> String{
    let mut encoded = String::new();
    // const reserved: &str = "!*'();:@&=+$,/?#[]";
    const UNRESERVED: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";

    for character in input.chars() {
        if UNRESERVED.contains(character) {
            encoded += &character.to_string();
            continue
        }

        let mut buffer: [u8; 4] = [0; 4];
        character.encode_utf8(&mut buffer);
        encoded += &buffer.iter().enumerate().filter(|&(_, x)| *x != 0).map(|(_,v)| format!("%{:X}", v)).collect::<Vec<String>>().join("");

    }
    
    return encoded;
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
