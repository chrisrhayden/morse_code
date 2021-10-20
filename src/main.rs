#![allow(dead_code)]

mod morse_code_key;
mod morse_printer;

use std::env::args;

use morse_code_key::{ascii_to_morse, morse_to_ascii};

#[derive(Debug, Clone)]
enum Morse {
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

fn unpack_morse_value(morse_char: &[u64]) -> char {
    let mut morse_char_value = 0;
    for (i, v) in morse_char.iter().enumerate() {
        if i != 0 {
            morse_char_value <<= 2;
        }
        morse_char_value += v;
    }

    morse_to_ascii(morse_char_value)
}

fn unpack_morse_code(code_vec: Vec<u64>) -> String {
    let mask = 0b11;
    let mut message = String::new();
    let mut morse_char: Vec<u64> = vec![];

    for mut code in code_vec.into_iter().rev() {
        while code != 0 {
            let seq = code & mask;
            code >>= 2;
            if seq == 0b01 {
                if !morse_char.is_empty() {
                    let ascii_char = unpack_morse_value(&morse_char);

                    message.push(ascii_char);

                    morse_char.clear();
                }
            } else {
                morse_char.insert(0, seq);
            }
        }

        if !morse_char.is_empty() {
            let ascii_char = unpack_morse_value(&morse_char);
            morse_char.clear();
            message.push(ascii_char);
        }
    }

    message
}

fn make_morse_code(msg: &str) -> Vec<u64> {
    let mask = 9223372036854775808;
    let mut morse_code: Vec<u64> = vec![0];

    let mut indx = 0;

    let mut code = &mut morse_code[indx];
    for c in msg.chars() {
        let (morse_value, offset) = ascii_to_morse(c);
        if *code << offset & mask > 0 {
            morse_code.push(0);
            indx += 1;
            code = &mut morse_code[indx];
        }

        *code <<= offset;
        *code += morse_value;
        *code <<= 2;
        *code += 1;
    }

    println!("{:?}", morse_code);
    morse_code
}

fn main() {
    let mut print = false;
    for arg in args() {
        if arg == "--print" || arg == "-p" {
            print = true;
        }
    }

    if print {
        morse_printer::print_morse_values();
    } else {
        let msg = "this is a long string to encode";
        let morse_code = make_morse_code(&msg);
        let new_msg = unpack_morse_code(morse_code);

        println!(">{}<", new_msg.chars().rev().collect::<String>());
    }
}
