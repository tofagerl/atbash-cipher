use std::collections::HashMap;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
const AHPLA: &str = "zyxwvutsrqponmlkjihgfedcba";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded_string = codec(plain);
    add_spaces(&encoded_string)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    codec(cipher)
}

fn codec(text: &str) -> String {
    let alphabet: HashMap<char, char> = ALPHA.chars().zip(AHPLA.chars()).collect();
    let mut result = String::new();
    for x in text.chars() {
        if x.is_ascii_alphabetic() {
            let c = x.to_ascii_lowercase();
            let encoded_char = alphabet
                .get(&c)
                .unwrap_or_else(|| panic!(format!("character {} not found in array", &c)));
            result.push(*encoded_char);
        } else if x.is_ascii_alphanumeric() && x != ' ' {
            result.push(x);
        }
    }
    result
}

fn add_spaces(string: &str) -> String {
    string.chars().enumerate().fold(String::new(), |mut acc, (count, c)|{
       if count %5 == 0{
           acc.push(' ');
       }
        acc.push(c);
        acc
    }).trim().to_string()
}
