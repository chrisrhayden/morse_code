use std::fmt::Display;

use crate::morse_code_key::binary_to_morse_key;

#[derive(Debug, Clone)]
pub enum Morse {
    Dit,
    Dah,
    Space,
}

impl Morse {
    fn value(&self) -> u64 {
        match self {
            Morse::Dit => 2,
            Morse::Dah => 3,
            Morse::Space => 0,
        }
    }
}

impl Display for Morse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let morse_str = match self {
            Morse::Dit => "Dit",
            Morse::Dah => "Dah",
            Morse::Space => "Space",
        };

        f.write_str(morse_str)
    }
}

fn pack_morse(code_vec: &[Morse]) -> (u64, u64) {
    let mut count = 0;
    let mut value: u64 = 0;
    for morse in code_vec.iter() {
        let code_value = morse.value();
        if value != 0 {
            value <<= 2;
        }
        value += code_value;
        count += 2;
    }
    (value, count)
}

fn ascii_to_morse(c: char) -> Vec<Morse> {
    match c {
        'a' => vec![Morse::Dit, Morse::Dah],
        'b' => vec![Morse::Dah, Morse::Dit, Morse::Dit, Morse::Dit],
        'c' => vec![Morse::Dah, Morse::Dit, Morse::Dah, Morse::Dit],
        'd' => vec![Morse::Dah, Morse::Dit, Morse::Dit],
        'e' => vec![Morse::Dit],
        'f' => vec![Morse::Dit, Morse::Dit, Morse::Dah, Morse::Dit],
        'g' => vec![Morse::Dah, Morse::Dah, Morse::Dit],
        'h' => vec![Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dit],
        'i' => vec![Morse::Dit, Morse::Dit],
        'j' => vec![Morse::Dit, Morse::Dah, Morse::Dah, Morse::Dah],
        'k' => vec![Morse::Dah, Morse::Dit, Morse::Dah],
        'l' => vec![Morse::Dit, Morse::Dah, Morse::Dit, Morse::Dit],
        'm' => vec![Morse::Dah, Morse::Dah],
        'n' => vec![Morse::Dah, Morse::Dit],
        'o' => vec![Morse::Dah, Morse::Dah, Morse::Dah],
        'p' => vec![Morse::Dit, Morse::Dah, Morse::Dah, Morse::Dit],
        'q' => vec![Morse::Dah, Morse::Dah, Morse::Dit, Morse::Dah],
        'r' => vec![Morse::Dit, Morse::Dah, Morse::Dit],
        's' => vec![Morse::Dit, Morse::Dit, Morse::Dit],
        't' => vec![Morse::Dah],
        'u' => vec![Morse::Dit, Morse::Dit, Morse::Dah],
        'v' => vec![Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dah],
        'w' => vec![Morse::Dit, Morse::Dah, Morse::Dah],
        'x' => vec![Morse::Dah, Morse::Dit, Morse::Dit, Morse::Dah],
        'y' => vec![Morse::Dah, Morse::Dit, Morse::Dah, Morse::Dah],
        'z' => vec![Morse::Dah, Morse::Dah, Morse::Dit, Morse::Dit],
        ' ' => vec![Morse::Space],
        _ => panic!("bad char {}", c),
    }
}

fn ascii_to_morse_print(msg: &str) {
    println!(
        "pub fn ascii_to_morse(c: char) -> (u64, u64) {}\n    match c {}",
        "{", "{"
    );
    for c in msg.chars() {
        let morse_value = ascii_to_morse(c);
        let (packed_morse, offset) = pack_morse(&morse_value);
        println!("        '{}' => ({}, {}),", c, packed_morse, offset);
    }
    println!(
        "        _ => panic!(\"bad char {}\", c)\n    {}\n{}",
        "{}", "}", "}"
    );
}

fn morse_to_ascii_print(msg: &str) {
    println!(
        "pub fn morse_to_ascii(code: u64) -> char {}\n    match code {}",
        "{", "{"
    );
    for c in msg.chars() {
        let morse_value = ascii_to_morse(c);
        let (packed_morse, _) = pack_morse(&morse_value);
        println!("        {:3} => '{}',", packed_morse, c);
    }
    println!(
        "        _ => panic!(\"bad morse value {}\", code)\n    {}\n{}",
        "{}", "}", "}"
    );
}

fn binary_morse_to_enum_morse(msg: &str) {
    println!(
        "pub fn binary_to_morse_key(code: u64) -> Vec<Morse> {}\n    match code {}",
        "{", "{"
    );
    for c in msg.chars() {
        let morse_value = ascii_to_morse(c);
        let (packed_morse, _) = pack_morse(&morse_value);
        print!("        {:3} => vec![", packed_morse);
        for (i, letter_code) in morse_value.iter().enumerate() {
            if i != morse_value.len() - 1 {
                print!("Morse::{}, ", letter_code);
            } else {
                print!("Morse::{}", letter_code);
            }
        }

        println!("],");
    }
    println!(
        "          _ => panic!(\"bad morse value {}\", code)\n    {}\n{}",
        "{}", "}", "}"
    );
}

pub fn print_morse_key() {
    let word = "abcdefghijklmnopqrstuvwxyz ";
    ascii_to_morse_print(&word);
    println!();
    morse_to_ascii_print(&word);
    println!();
    binary_morse_to_enum_morse(&word);
}

fn unpack_to_individual_codes(code: &[u64]) -> Vec<u64> {
    let mask = 0b11;

    let mut letter: Vec<u64> = vec![];

    let mut codes: Vec<u64> = vec![];

    for num in code.iter().rev() {
        let mut num = num.clone();

        while num != 0 {
            let sequence = num & mask;

            num >>= 2;

            if sequence == 0b01 {
                if !letter.is_empty() {
                    let mut whole_letter = 0;

                    for l_num in letter.iter().rev() {
                        whole_letter <<= 2;
                        whole_letter += l_num;
                    }

                    codes.push(whole_letter);

                    letter.clear();
                }
            } else {
                letter.push(sequence);
            }
        }

        if !letter.is_empty() {
            let mut whole_letter = 0;

            for l_num in letter.iter().rev() {
                whole_letter <<= 2;
                whole_letter += l_num;
            }

            codes.push(whole_letter);
            letter.clear();
        }
    }

    codes.into_iter().rev().collect::<Vec<u64>>()
}

fn print_code_as_group(letters: &[u64]) {
    print!("binary  = ");
    for num in letters.iter() {
        print!("{:08b} ", num)
    }
    println!();
}

fn print_code_as_number(code: &[u64]) {
    print!("number  = [");
    for (i, num) in code.iter().enumerate() {
        if i != code.len() - 1 {
            print!("{}, ", num);
        } else {
            print!("{}", num);
        }
    }
    println!("]");
}

fn print_as_readable_code(letters: &[u64]) {
    print!("morse   = ");
    for (i, num) in letters.iter().enumerate() {
        let morse_vec = binary_to_morse_key(*num);

        print!("[");
        for (j, value) in morse_vec.iter().enumerate() {
            if j != morse_vec.len() - 1 {
                print!("{}, ", value);
            } else {
                print!("{}", value);
            }
        }

        if i != letters.len() - 1 {
            print!("], ");
        } else {
            print!("]");
        }
    }

    println!();
}

pub fn print_morse_code(code: &[u64], message: &str) {
    print_code_as_number(code);
    let letters = unpack_to_individual_codes(code);
    print_code_as_group(&letters);
    print_as_readable_code(&letters);

    println!("message = {}", message.chars().rev().collect::<String>());
}
