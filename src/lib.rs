pub fn reverse_string(s: &str) -> String {
    if s == "" {
        return "".into();
    }

    let bytes = s.as_bytes();
    let mut result = allocate_byte_array(bytes.len());
    let mut i_src = 0;

    while i_src < bytes.len() {
        let i_end = {
            let mut value = i_src;
            loop {
                if value == bytes.len() - 1 || is_first_byte(bytes[value + 1]) {
                    break value
                }
                value += 1;
            }
        };


        let char_length = i_end - i_src + 1;
        let src = &bytes[i_src..][..char_length];
        let dst = &mut result[bytes.len() - i_src - char_length..][..char_length];
        dst.copy_from_slice(src);

        i_src = i_end + 1;
    }
    unsafe {
        String::from_utf8_unchecked(result)
    }
}

fn allocate_byte_array(len: usize) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(len);
    unsafe {
        v.set_len(len);
    }
    v
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
