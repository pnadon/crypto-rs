use std::fs::File;
use std::io::prelude::*;
extern crate crypto;
use crypto::{
    repeating_key_xor
};
fn main() -> std::io::Result<()> {
    let mut file = File::open("6.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let input = &String::from_utf8(
        base64::decode(contents).unwrap()
    ).unwrap();
    let keys = repeating_key_xor::break_repeating_key_xor(
        input, 
        2, 
        40, 
        10
    );
    for key in keys {
        println!("{:?}", String::from_utf8(repeating_key_xor::repeating_key_xor(input, &key)));
    }
    Ok(())
}
