fn reverse_string(s: &str) -> String {
    if s == "" {
        return "".into();
    }

    let bytes = s.as_bytes();
    let mut result = vec![0; bytes.len()];
    let mut i = 0;
    let mut j = bytes.len() - 1;
    while i <= j {
        let i_end = loop {
            let mut value = i;
            if is_first_byte(bytes[value + 1]) {
                break value
            }
            value += 1;
        };

        let j_start = loop {
            let mut value = j;
            if is_first_byte(bytes[value - 1]) {
                break value
            }
            value -= 1;
        };

        let first_char_length = i_end - i + 1;
        let last_char_length = j - j_start + 1;

        result[i..i+last_char_length].copy_from_slice(&bytes[j_start..j_start + last_char_length]);
        result[j_start..j_start+first_char_length].copy_from_slice(&bytes[i..i+first_char_length]);

        i = i_end + 1;
        j = j_start - 1;
    }
    String::from_utf8(result).unwrap()
}

fn is_first_byte(value: u8) -> bool {
    !((value & 0b10000000) == 0b10000000 && (value & 0b01000000) == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_byte() {
        assert!(!is_first_byte(0b10101010));
        assert!(!is_first_byte(0b10111110));
        assert!(is_first_byte(0b00111110));
        assert!(is_first_byte(0b01111110));
    }

    #[test]
    fn reverse_even() {
        assert_eq!("abcdef", &reverse_string("fedcba"));
    }

    #[test]
    fn reverse_odd() {
        assert_eq!("olleh", &reverse_string("hello"));
    }

    #[test]
    fn reverse_empty() {
        assert_eq!("", &reverse_string(""));
    }
}
