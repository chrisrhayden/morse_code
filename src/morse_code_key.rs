use crate::morse_printer::Morse;

pub fn ascii_to_morse(c: char) -> (u64, u64) {
    match c {
        'a' => (11, 4),
        'b' => (234, 8),
        'c' => (238, 8),
        'd' => (58, 6),
        'e' => (2, 2),
        'f' => (174, 8),
        'g' => (62, 6),
        'h' => (170, 8),
        'i' => (10, 4),
        'j' => (191, 8),
        'k' => (59, 6),
        'l' => (186, 8),
        'm' => (15, 4),
        'n' => (14, 4),
        'o' => (63, 6),
        'p' => (190, 8),
        'q' => (251, 8),
        'r' => (46, 6),
        's' => (42, 6),
        't' => (3, 2),
        'u' => (43, 6),
        'v' => (171, 8),
        'w' => (47, 6),
        'x' => (235, 8),
        'y' => (239, 8),
        'z' => (250, 8),
        ' ' => (0, 2),
        _ => panic!("got bad char {}", c),
    }
}

pub fn morse_to_ascii(code: u64) -> char {
    match code {
        11 => 'a',
        234 => 'b',
        238 => 'c',
        58 => 'd',
        2 => 'e',
        174 => 'f',
        62 => 'g',
        170 => 'h',
        10 => 'i',
        191 => 'j',
        59 => 'k',
        186 => 'l',
        15 => 'm',
        14 => 'n',
        63 => 'o',
        190 => 'p',
        251 => 'q',
        46 => 'r',
        42 => 's',
        3 => 't',
        43 => 'u',
        171 => 'v',
        47 => 'w',
        235 => 'x',
        239 => 'y',
        250 => 'z',
        0 => ' ',
        _ => panic!("bad morse value"),
    }
}

pub fn binary_to_morse_key(code: u64) -> Vec<Morse> {
    match code {
        11 => vec![Morse::Dit, Morse::Dah],
        234 => vec![Morse::Dah, Morse::Dit, Morse::Dit, Morse::Dit],
        238 => vec![Morse::Dah, Morse::Dit, Morse::Dah, Morse::Dit],
        58 => vec![Morse::Dah, Morse::Dit, Morse::Dit],
        2 => vec![Morse::Dit],
        174 => vec![Morse::Dit, Morse::Dit, Morse::Dah, Morse::Dit],
        62 => vec![Morse::Dah, Morse::Dah, Morse::Dit],
        170 => vec![Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dit],
        10 => vec![Morse::Dit, Morse::Dit],
        191 => vec![Morse::Dit, Morse::Dah, Morse::Dah, Morse::Dah],
        59 => vec![Morse::Dah, Morse::Dit, Morse::Dah],
        186 => vec![Morse::Dit, Morse::Dah, Morse::Dit, Morse::Dit],
        15 => vec![Morse::Dah, Morse::Dah],
        14 => vec![Morse::Dah, Morse::Dit],
        63 => vec![Morse::Dah, Morse::Dah, Morse::Dah],
        190 => vec![Morse::Dit, Morse::Dah, Morse::Dah, Morse::Dit],
        251 => vec![Morse::Dah, Morse::Dah, Morse::Dit, Morse::Dah],
        46 => vec![Morse::Dit, Morse::Dah, Morse::Dit],
        42 => vec![Morse::Dit, Morse::Dit, Morse::Dit],
        3 => vec![Morse::Dah],
        43 => vec![Morse::Dit, Morse::Dit, Morse::Dah],
        171 => vec![Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dah],
        47 => vec![Morse::Dit, Morse::Dah, Morse::Dah],
        235 => vec![Morse::Dah, Morse::Dit, Morse::Dit, Morse::Dah],
        239 => vec![Morse::Dah, Morse::Dit, Morse::Dah, Morse::Dah],
        250 => vec![Morse::Dah, Morse::Dah, Morse::Dit, Morse::Dit],
        0 => vec![Morse::Space],
        _ => panic!("bad morse value {}", code),
    }
}
