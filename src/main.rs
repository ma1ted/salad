fn alphabet(start: &u8, end: &u8) -> Vec<char> {
    let mut alphabet: Vec<char> = Vec::new();
    for i in *start..*end {
        let i_char: char = char::from_u32(i as u32).unwrap();
        alphabet.push(i_char);
    }
    alphabet
}

fn encrypt(encrypt: bool) -> String {
    let ascii_start: u8 = 32;
    let ascii_end: u8 = 127;
    let cleartext = "uif!rvjdl!cspxo!gpy!kvnqt!pwfs!uif!mb{z!eph\"";
    let shift = 1;

    let alphabet: Vec<char> = alphabet(&ascii_start, &ascii_end);
    let alphabet_len: i32 = alphabet.len() as i32;

    let mut cyphertext = String::new();
    for c in cleartext.chars() {
        if !alphabet.contains(&c) {
            panic!("All cleartext input characters' decimal ASCII values must be within the alphabet ranges supplied");
        }
        let index: i32 = alphabet.iter().position(|&r| r == c).unwrap() as i32;

        if encrypt {
            let mut index_new = index + shift;
            while index_new > alphabet_len - 1 {
                index_new -= alphabet_len;
            }
            cyphertext.push(alphabet[index_new as usize]);
        } else {
            let mut index_new = index - shift;
            while index_new < 0 {
                index_new += alphabet_len;
            }
            cyphertext.push(alphabet[index_new as usize]);
        }
    }
    cyphertext
}

fn main() {
    let result = encrypt(false);
    println!("{}", result);
}
