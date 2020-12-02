pub mod base64;
pub mod heuristics;
pub mod hex;
pub mod repeating_key_xor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
