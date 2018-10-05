fn reverse_string(s: &str) -> String {
    if s == "" {
        return "".into();
    }

    let bytes = s.as_bytes();
    let mut result = vec![0; bytes.len()];
    let mut i_src = 0;
    let mut j_src = bytes.len() - 1;
    let mut i_dst = i_src;
    let mut j_dst = j_src;

    while i_src <= j_src {
        let i_end = {
            let mut value = i_src;
            loop {
                if is_first_byte(bytes[value + 1]) {
                    break value
                }
                value += 1;
            }
        };

        let j_start = {
            let mut value = j_src;
            loop {
                if is_first_byte(bytes[value]) {
                    break value
                }
                value -= 1;
            }
        };

        let first_char_length = i_end - i_src + 1;
        let last_char_length = j_src - j_start + 1;

        result[i_dst..][..last_char_length].copy_from_slice(&bytes[j_start..][..last_char_length]);
        result[j_dst - first_char_length + 1..][..first_char_length].copy_from_slice(&bytes[i_src..][..first_char_length]);

        i_src = i_end + 1;
        j_src = j_start - 1;
        i_dst += last_char_length;
        j_dst -= first_char_length;
    }
    String::from_utf8(result).unwrap()
}

fn is_first_byte(value: u8) -> bool {
    !((value & 0b10000000) == 0b10000000 && (value & 0b01000000) == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_char {
        ($c:expr, $test:ident) => {
            #[test]
            fn $test() {
                let bytes = $c.as_bytes();
                assert!(is_first_byte(bytes[0]), "First byte");
                for i in 1..bytes.len() {
                    assert!(!is_first_byte(bytes[i]), format!("Byte #{}", i));
                }
            }
        }
    }

    test_char!("😮", four_byte);
    test_char!("€", three_byte);
    test_char!("¢", two_byte);

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

    #[test]
    fn reverse_unicode() {
        assert_eq!("€$¢😮", &reverse_string("😮¢$€"));
    }
}
