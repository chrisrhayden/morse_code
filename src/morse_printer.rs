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

pub fn print_morse_key() {
    let word = "abcdefghijklmnopqrstuvwxyz ";
    ascii_to_morse_print(&word);
    println!();
    morse_to_ascii_print(&word);
}

pub fn print_morse_code(code: &[u64], message: &str) {
    print!("number  = [");
    for (i, num) in code.iter().enumerate() {
        if i != code.len() - 1 {
            print!("{}, ", num);
        } else {
            print!("{}", num);
        }
    }
    print!("]\nbinary  = ");

    for num in code {
        print!("{:b}", num);
    }
    println!("\nmessage = {}", message.chars().rev().collect::<String>());
}
