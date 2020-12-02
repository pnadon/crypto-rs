pub fn hamming_distance(str1: &str, str2: &str) -> usize {
    assert_eq!(str1.len(), str2.len());
    str1.bytes().zip(str2.bytes())
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .fold(0, |mut acc, mut num| {
            while num != 0 {
                num &= num - 1;
                acc += 1;
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use core::panic;
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            hamming_distance("this is a test", "wokka wokka!!!"),
            37
        )
    }
}