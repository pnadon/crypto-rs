use std::collections::HashMap;
use crate::{
    hex::{
        hex_to_bytes,
        bytes_to_hex
    },
    heuristics::hamming_distance,
};

fn xor_hex(input_1: &str, input_2: &str) -> String {
    bytes_to_hex(
        hex_to_bytes(input_1).into_iter()
            .zip(hex_to_bytes(input_2).into_iter())
            .map(|(x, y)| x ^ y)
            .collect::<Vec<u8>>()
    )
}

pub fn xor_key(bytes: Vec<u8>) -> Option<char> {
    let e_val = ' ' as u8; // space is the most common char
    let mut char_freqs = HashMap::new();
    for byte in &bytes {
        char_freqs
            .entry(*byte)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    while !char_freqs.is_empty() {
        let max_chr = *char_freqs.iter()
            .max_by(|e1, e2| e1.1.cmp(e2.1)).unwrap().0;
        let key = e_val ^ max_chr;
        if no_invalid_chars(&bytes, key) {
            return Some(key as char);
        }
        char_freqs.remove(&max_chr);
    }
    None
}

fn no_invalid_chars(bytes: &Vec<u8>, key: u8) -> bool {
    bytes
        .iter().map(|c| (c ^ key) as char)
        .filter(|c| c != &'\n')
        .min().unwrap() >= ' '
}

pub fn repeating_key_xor(input: &str, key: &str) -> Vec<u8> {
        input.chars()
            .zip(key.chars().cycle())
            .map(|(input_char, key_char)| (input_char as u8) ^ (key_char as u8))
            .collect::<Vec<u8>>()
}

pub fn keysize_dists(input: &str, min_guess: usize, max_guess: usize) -> Vec<usize> {
    let max_guess = std::cmp::min(max_guess, input.len() / 2);
    (min_guess..max_guess).map(|keysize|
        hamming_distance(&input[..keysize], &input[keysize..(keysize*2)])
    ).collect()

}

pub fn break_repeating_key_xor(input: &str, min_keysize: usize, max_keysize: usize, num_tries: usize) -> Vec<String> {
    let mut dists = keysize_dists(input, min_keysize, max_keysize);

    (0..num_tries).map(|_| {
        let min_dist = dists.iter().min().unwrap();
        let keysize = dists.iter().position(|elem| elem == min_dist).unwrap() + min_keysize;
        dists.remove(keysize - min_keysize);

        let trans_blocks = transpose_blocks(input, keysize);

        let key = trans_blocks.into_iter()
            .map(move |block| {
                    xor_key(block)
            })
            .collect::<Vec<Option<char>>>();
        
        match key.iter().all(|c| c.is_some()) {
            true => Some(key.iter().map(|c| c.unwrap()).collect::<String>()),
            false => None,
        }
    }).filter_map(|key| key)
    .collect::<Vec<String>>()
}

fn transpose_blocks(input: &str, keysize: usize) -> Vec<Vec<u8>> {
    let mut trans_blocks = (0..keysize)
        .map(|_| vec![])
        .collect::<Vec<Vec<u8>>>();
    
    input.bytes().collect::<Vec<u8>>()
        .chunks(keysize)
        .for_each(|chunk| {
            for (i, val) in chunk.iter().enumerate() {
                trans_blocks[i].push(*val);
            }
        }
    );

    trans_blocks
}

#[cfg(test)]
mod tests {
    use core::panic;
    use super::*;

    #[test]
    fn test_xor_hex() {
        assert_eq!(
            xor_hex(
                "1c0111001f010100061a024b53535009181c", 
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        )
    }

    #[test]
    fn test_repeating_key_xor() {
        assert_eq!(
            bytes_to_hex(repeating_key_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE")),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
    }

    #[test]
    fn test_transpose_blocks() {
        let input = "hellobbbbbaaaaapqrst";
        assert_eq!(
            transpose_blocks(input, 5)[1],
            vec![b'e', b'b', b'a', b'q'],
        )
    }
}