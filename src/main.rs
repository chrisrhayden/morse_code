#![allow(dead_code)]

mod morse_code_key;
mod morse_printer;

use std::{char::from_u32, env::args, error::Error};

use morse_code_key::{ascii_to_morse, morse_to_ascii};
use morse_printer::{print_morse_code, print_morse_key};

struct Args {
    print_key: bool,
    message: Option<String>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            print_key: false,
            message: None,
        }
    }
}

fn convert_to_ascii(morse_char: &[u64]) -> char {
    let mut morse_char_value = 0;

    for v in morse_char {
        morse_char_value <<= 2;
        morse_char_value += v;
    }

    morse_to_ascii(morse_char_value)
}

fn unpack_morse_code(code_vec: &[u64]) -> String {
    let mask = 0b11;

    let mut message = String::new();
    let mut morse_char: Vec<u64> = vec![];

    for code_immutable in code_vec.iter().rev() {
        let mut code = code_immutable.clone();

        while code != 0 {
            let sequence = code & mask;

            code >>= 2;

            if sequence == 0b01 {
                if !morse_char.is_empty() {
                    let ascii_char = convert_to_ascii(&morse_char);
                    message.push(ascii_char);
                    morse_char.clear();
                }
            } else {
                morse_char.insert(0, sequence);
            }
        }

        if !morse_char.is_empty() {
            let ascii_char = convert_to_ascii(&morse_char);
            message.push(ascii_char);
            morse_char.clear();
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
        let mut c_ascii = c as u32;
        if c_ascii != 32
            && ((c_ascii < 65 || c_ascii > 122)
                || (c_ascii > 91 && c_ascii < 97))
        {
            eprintln!("skipping bad char {}", c);
            break;
        } else if c_ascii != 32 && c_ascii < 97 {
            c_ascii += 32;
        }

        let (morse_value, offset) = ascii_to_morse(from_u32(c_ascii).unwrap());

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

    morse_code
}

fn parse_args() -> Result<Args, Box<dyn Error>> {
    let mut arg_options = Args::default();

    let mut get_msg = false;

    for arg in args().skip(1) {
        if cfg!(debug_assertions) {
            dbg!(&arg);
        }

        if arg == "--print-key" || arg == "-k" {
            arg_options.print_key = true;
        } else if arg == "--msg" || arg == "-m" {
            get_msg = true;
        } else if get_msg {
            arg_options.message = Some(arg);
            get_msg = false;
        } else {
            return Err(Box::from(format!("bad arg -- {}", arg)));
        }
    }

    if !arg_options.print_key && arg_options.message.is_none() {
        Err(Box::from(String::from("did not get arguments")))
    } else {
        Ok(arg_options)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let arg_options = parse_args()?;

    if arg_options.print_key {
        if cfg!(debug_assertions) {
            dbg!("print morse code key");
        }

        print_morse_key();
    }
    if let Some(msg) = arg_options.message {
        if cfg!(debug_assertions) {
            dbg!(&msg);
        }

        let morse_code = make_morse_code(&msg);
        let decoded_msg = unpack_morse_code(&morse_code);
        print_morse_code(&morse_code, &decoded_msg);
    }

    Ok(())
}
