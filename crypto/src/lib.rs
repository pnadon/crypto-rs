pub mod base64;
pub mod heuristics;
pub mod hex;
pub mod repeating_key_xor;

#[cfg(test)]
mod tests {

    #[test]
    fn test_im_not_crazy() {
        assert_eq!(b'c', 'c' as u8);
    }
}
