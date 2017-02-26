use std::str::FromStr;

use error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Color> {
        let color = if s.starts_with('#') {
            let (alpha, rgb_offset) = if s.len() == 9 {
                (hex_pair_to_number(&s[1..3]), 3)
            } else {
                (Some(255), 1)
            };
            alpha.and_then(|a| hex_rgb_to_rgb(&s[rgb_offset..]).and_then(|(r, g, b)| Some(Color(a, r, g, b))))
        } else {
            hex_rgb_to_rgb(s).and_then(|(r, g, b)| Some(Color(255, r, g, b)))
        };
        color.ok_or(Error::InvalidColor(s.to_string()))
    }
}

fn hex_char_to_number(c: char) -> Option<u8> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'a' | 'A' => Some(10),
        'b' | 'B' => Some(11),
        'c' | 'C' => Some(12),
        'd' | 'D' => Some(13),
        'e' | 'E' => Some(14),
        'f' | 'F' => Some(15),
        _ => None,
    }
}

fn hex_pair_to_number(s: &str) -> Option<u8> {
    let mut chars = s.chars().flat_map(hex_char_to_number);
    chars.next().and_then(|n1| chars.next().and_then(|n2| Some(16u8 * n1 + n2)))
}

fn hex_rgb_to_rgb(s: &str) -> Option<(u8, u8, u8)> {
    if s.len() == 6 {
        hex_pair_to_number(&s[0..2]).and_then(|r| {
            hex_pair_to_number(&s[2..4])
                .and_then(|g| hex_pair_to_number(&s[4..6]).and_then(|b| Some((r, g, b))))
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_hex_char_to_number() {
        assert_eq!(Some(0), hex_char_to_number('0'));
        assert_eq!(Some(1), hex_char_to_number('1'));
        assert_eq!(Some(15), hex_char_to_number('f'));
        assert_eq!(None, hex_char_to_number('g'));
    }

    #[test]
    fn test_hex_pair_to_number() {
        assert_eq!(None, hex_pair_to_number(""));
        assert_eq!(None, hex_pair_to_number("0"));
        assert_eq!(Some(0), hex_pair_to_number("00"));
        assert_eq!(Some(255), hex_pair_to_number("ff"));
        assert_eq!(None, hex_pair_to_number("xy"));
    }

    #[test]
    fn test_hex_rgb_to_rgb() {
        assert_eq!(None, hex_rgb_to_rgb("124"));
        assert_eq!(Some((1u8, 2u8, 4u8)), hex_rgb_to_rgb("010204"));
    }

    #[test]
    fn test_hex_string_to_color() {
        assert!(Color::from_str("").is_err());
        assert!(Color::from_str("010204").is_ok());
        assert!(Color::from_str("#010204").is_ok());
        assert!(Color::from_str("00010204").is_err());
        assert!(Color::from_str("#00010204").is_ok());
    }
}
