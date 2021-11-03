#![allow(dead_code)]

static ASCII_DEFAULT: (u8, u8) = (97, 123);
static ASCII_EXTENDED: (u8, u8) = (32, 127);

fn alphabet(start: &u8, end: &u8) -> Vec<char> {
    let mut alphabet: Vec<char> = Vec::new();
    for i in *start..*end {
        let i_char: char = char::from_u32(i as u32).unwrap();
        alphabet.push(i_char);
    }
    alphabet
}

fn _encrypt(cleartext: &str, shift: i32, preserve_whitespace: bool, ascii_start: u8, ascii_end: u8) -> String {
    let alphabet: Vec<char> = alphabet(&ascii_start, &ascii_end);
    let alphabet_len: i32 = alphabet.len() as i32;

    let mut cyphertext = String::new();
    for c in cleartext.chars() {
        if c == ' ' {
            if preserve_whitespace {
                cyphertext.push(' ');
                continue;
            }
            if ascii_end == ASCII_DEFAULT.1 {
                panic!("The input text can not contain whitespaces when 'preserve whitespaces' or 'extended characterset' is not enabled");
            }
        }

        if !alphabet.contains(&c) {
            panic!("All cleartext input characters' decimal ASCII values must be within the alphabet ranges supplied");
        }
        let mut index: i32 = alphabet.iter().position(|&r| r == c).unwrap() as i32;

        index += shift;
        
        while index > alphabet_len - 1 {
            index -= alphabet_len;
        }
        cyphertext.push(alphabet[index as usize]);
    }
    cyphertext
}

fn _decrypt(cyphertext: &str, shift: i32, preserve_whitespace: bool, ascii_start: u8, ascii_end: u8) -> String {
    let alphabet: Vec<char> = alphabet(&ascii_start, &ascii_end);
    let alphabet_len: i32 = alphabet.len() as i32;

    let mut cleartext = String::new();
    for c in cyphertext.chars() {
        if c == ' ' {
            if preserve_whitespace {
                cleartext.push(' ');
                continue;
            }
            if ascii_end == ASCII_DEFAULT.1 {
                panic!("The input text can not contain whitespaces when 'preserve whitespaces' or 'extended characterset' is not enabled");
            }
        }

        if !alphabet.contains(&c) {
            panic!("All cyphertext input characters' decimal ASCII values must be within the alphabet ranges supplied");
        }
        let mut index: i32 = alphabet.iter().position(|&r| r == c).unwrap() as i32;

        index -= shift;

        while index < 0 {
            index += alphabet_len;
        }
        cleartext.push(alphabet[index as usize]);
    }
    cleartext
}



pub fn encrypt(cleartext: &str, shift: i32, preserve_whitespace: bool) -> String { _encrypt(cleartext, shift, preserve_whitespace, ASCII_DEFAULT.0, ASCII_DEFAULT.1) }

pub fn encrypt_extended(cleartext: &str, shift: i32, preserve_whitespace: bool) -> String { _encrypt(cleartext, shift, preserve_whitespace, ASCII_EXTENDED.0, ASCII_EXTENDED.1) }

pub fn encrypt_custom(cleartext: &str, shift: i32, preserve_whitespace: bool, ascii_start: u8, ascii_end: u8) -> String { _encrypt(cleartext, shift, preserve_whitespace, ascii_start, ascii_end) }


pub fn decrypt(cyphertext: &str, shift: i32, preserve_whitespace: bool) -> String { _decrypt(cyphertext, shift, preserve_whitespace, ASCII_DEFAULT.0, ASCII_DEFAULT.1) }

pub fn decrypt_extended(cyphertext: &str, shift: i32, preserve_whitespace: bool) -> String { _decrypt(cyphertext, shift, preserve_whitespace, ASCII_EXTENDED.0, ASCII_EXTENDED.1) }

pub fn decrypt_custom(cyphertext: &str, shift: i32, preserve_whitespace: bool, ascii_start: u8, ascii_end: u8) -> String { _decrypt(cyphertext, shift, preserve_whitespace, ascii_start, ascii_end) }
