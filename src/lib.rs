use std::collections::HashMap;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
const AHPLA: &str = "zyxwvutsrqponmlkjihgfedcba";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded_string = codec(plain);
    let result = add_spaces(encoded_string.as_str());

    result.trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    return codec(cipher);
}

fn codec(text: &str) -> String {
    let mut alphabet: HashMap<char, char> = HashMap::new();
    for i in 0..26 {
        alphabet.insert(ALPHA.chars().nth(i).unwrap(), AHPLA.chars().nth(i).unwrap());
    }

    let mut result = String::new();
    for x in text.chars() {
        if x.is_ascii_alphabetic() {
            let c = x.to_lowercase().next().unwrap();
            let encoded_char: &char = alphabet
                .get(&c)
                .unwrap_or_else(|| panic!("character {} not found in array", &c));
            result.push(encoded_char.clone());
        } else if x.is_ascii_alphanumeric() && x != ' ' {
            result.push(x);
        }
    }
    result.trim().to_string()
}

fn add_spaces(string: &str) -> String {
    let mut result = String::new();
    let mut it = 0;

    for x in string.chars(){
        if it%5==0 {
            result.push(' ');
        }
        result.push(x);
        it += 1;
    }
    result
}
