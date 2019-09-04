use std::collections::HashMap;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
const AHPLA: &str = "zyxwvutsrqponmlkjihgfedcba";


/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut alphabet: HashMap<char, char> = HashMap::new();
    for i in 0..26 {
        alphabet.insert(ALPHA.chars().nth(i).unwrap(), AHPLA.chars().nth(i).unwrap());
    }

    let mut result = String::with_capacity(plain.len());
    let mut iter: i8 = 0;
    for x in plain.chars() {
        if x.is_ascii_alphabetic() {
            let c = x.to_lowercase().next().unwrap();
            let encoded_char: &char = alphabet.get(&c).unwrap_or_else(|| panic!("character {} not found in array", &c));
            result.push(encoded_char.clone());
            iter += 1;
        } else if x.is_ascii_alphanumeric() && x != ' ' {
            result.push(x);
            iter += 1;
        }
        if iter == 5 {
            result.push(' ');
            iter = 0;
        }
    }
    result.trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    encode(cipher).replace(" ", "")
}