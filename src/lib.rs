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
    ALPHA.chars().zip(AHPLA.chars()).for_each(|(k, v)| {
        alphabet.insert(k, v);
    });

    let mut result = String::new();
    for x in text.chars() {
        if x.is_ascii_alphabetic() {
            let c = x.to_ascii_lowercase();
            let encoded_char: &char = alphabet
                .get(&c)
                .expect(format!("character {} not found in array", &c).as_ref());
            result.push(*encoded_char);
        } else if x.is_ascii_alphanumeric() && x != ' ' {
            result.push(x);
        }
    }
    result.trim().to_string()
}

fn add_spaces(string: &str) -> String {
    string.chars().enumerate().fold(String::new(), |acc, (count, c)|{
       if count %5 == 0{
           format!("{} {}", acc, c)
       } else {
           format!("{}{}", acc, c)
       }
    })
}
