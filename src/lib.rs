fn cipher_char(char: char, amount: i8) -> char {
    let last_char = if char.is_ascii_uppercase() {
        91 as u8
    } else {
        123 as u8
    };

    let first_char = last_char.abs_diff(26 as u8);

    if !&char.is_ascii_alphabetic() {
        panic!("Input is not an ASCII alphabetic character");
    }

    let c = (char as u8)
        .checked_add_signed(amount)
        .unwrap();
    if c > last_char {
        ((c % last_char) + first_char) as char
    } else if c < first_char {
        (last_char - first_char.abs_diff(c)) as char
    } else {
        c as char
    }
}

pub fn caeser(string: &str, amount: i8) -> String {
    string.chars().map(|s| cipher_char(s, amount)).collect()
}
