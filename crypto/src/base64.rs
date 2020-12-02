pub fn bytes_to_base64(input: Vec<u8>) -> String {
    let mut input_bytes = input.iter();
    let mut ans = "".to_string();
    loop {
        let chunk = (
            input_bytes.next(),
            input_bytes.next(),
            input_bytes.next(),
        );
        let mut rem;
        if let Some(num) = chunk.0 {
            ans.push(digit_to_base64_char(num >> 2));
            rem = num % (1<<2);
        } else {
            break;
        }

        if let Some(num) = chunk.1 {
            ans.push(digit_to_base64_char((rem << 4)+ (num >> 4)));
            rem = num % (1 << 4);
        } else {
            ans.push(digit_to_base64_char(rem << 4));
            ans.push('=');
            ans.push('=');
            break;
        }

        if let Some(num) = chunk.2 {
            ans.push(digit_to_base64_char((rem << 2) + (num >> 6)));
            ans.push(digit_to_base64_char(num % (1 << 6)));
        } else {
            ans.push(digit_to_base64_char(rem << 2));
            ans.push('=');
            break;
        }
        
    }
    ans
}

fn digit_to_base64_char(digit: u8) -> char {
    match digit {
        0..=25 => ('A' as u8 + digit) as char,
        26..=51 => ('a' as u8 + (digit - 26)) as char,
        52..=61 => ('0' as u8 + (digit - 52)) as char,
        62 => '+',
        63 => '/',
        _ => panic!(format!("Invalid digit for base64: {:?}", digit)),
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use crate::hex::hex_to_bytes;

    use super::*;

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(
            bytes_to_base64(
                hex_to_bytes(
                    "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
                )
            ),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}