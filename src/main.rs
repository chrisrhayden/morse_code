#![allow(dead_code)]

// https://pastebin.com/SXaJQyv2
mod morse_printer;
use morse_printer::print_morse_values;

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

fn pack_morse(code_vec: &[Morse]) -> u64 {
    let mut value: u64 = 0;
    for morse in code_vec.iter().rev() {
        let code_value = morse.value();
        if value != 0 {
            value <<= 2;
        }
        value += code_value;
    }
    value <<= 2;
    value += 1;
    value
}

fn unpack_morse(mut code: u64) -> String {
    let mask = 3;
    let mut message = String::new();
    let mut morse_char = 0;
    while code != 0 {
        let seq = code & mask;
        code >>= 2;
        println!("{}", seq);
        if seq == 1 {
            let ascii_value = morse_to_ascii(morse_char);
            message.push(ascii_value);
            morse_char = 0;
        } else {                
            morse_char += seq;
        }
    }

    message
}


fn make_morse_code(msg: &str) -> u64 {
    let mut morse_code = 0;
    for c in msg.chars() {
        let morse_value = ascii_to_morse(c);
        morse_code <<= 2;
        morse_code += morse_value;
        morse_code += 1;
    }

    morse_code
}


fn ascii_to_morse(c: char) -> u64 {
    match c {
        'a' =>  14,
        'b' => 171,
        'c' => 187,
        'd' =>  43,
        'e' =>   2,
        'f' => 186,
        'g' =>  47,
        'h' => 170,
        'i' =>  10,
        'j' => 254,
        'k' =>  59,
        'l' => 174,
        'm' =>  15,
        'n' =>  11,
        'o' =>  63,
        'p' => 190,
        'q' => 239,
        'r' =>  46,
        's' =>  42,
        't' =>   3,
        'u' =>  58,
        'v' => 234,
        'w' =>  62,
        'x' => 235,
        'y' => 251,
        'z' => 175,
        ' ' =>   0,
        _ => unreachable!()
    }
}

fn morse_to_ascii(code: u64) -> char {
    match code {
         14 => 'a',
        171 => 'b',
        187 => 'c',
         43 => 'd',
          2 => 'e',
        186 => 'f',
         47 => 'g',
        170 => 'h',
         10 => 'i',
        254 => 'j',
         59 => 'k',
        174 => 'l',
         15 => 'm',
         11 => 'n',
         63 => 'o',
        190 => 'p',
        239 => 'q',
         46 => 'r',
         42 => 's',
          3 => 't',
         58 => 'u',
        234 => 'v',
         62 => 'w',
        235 => 'x',
        251 => 'y',
        175 => 'z',
          0 => ' ',
        _ => unreachable!()
    }
}

fn main() {
    let msg = "hello world";
    let morse_code = make_morse_code(&msg);
    println!("{}", morse_code);
    let new_msg = unpack_morse(morse_code);
    println!("{}", new_msg);
}