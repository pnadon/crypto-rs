pub fn hex_to_bytes(input: &str) -> Vec<u8> {
    let chars = input.chars().collect::<Vec<char>>();
    chars
        .chunks(2)
        .map(|byte| (hex_to_num(byte[0]) << 4) + hex_to_num(byte[1]))
        .collect::<Vec<u8>>()
}

pub fn bytes_to_hex(input: Vec<u8>) -> String {
    input
        .iter()
        .fold("".to_string(), |mut acc, byte| {
            acc.push(num_to_hex(byte >> 4));
            acc.push(num_to_hex(byte % (1 << 4)));
            acc
        })
}

fn hex_to_num(input: char) -> u8 {
    let ans = match input {
        '0'..='9' => (input as u8 - '0' as u8),
        'a'..='f' => (input as u8 - 'a' as u8) + 10,
        'A'..='F' => (input as u8 - 'A' as u8) + 10,
        _ => panic!(format!("Invalid digit for hex: {:?}", input)),
    };
    assert!(ans < 16);
    ans
}

fn num_to_hex(input: u8) -> char {
    match input {
        0..=9 => ('0' as u8 + input) as char,
        10..=15 => ('a' as u8 + input - 10) as char,
        _ => panic!("number larger than hex value"),
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        let bytes = hex_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        assert_eq!(bytes[0], 0x49);
    }
}