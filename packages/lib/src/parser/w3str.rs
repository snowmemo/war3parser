use std::fmt::{Display, Formatter};

use nom::{
    bytes::complete::{tag_no_case, take, take_till1, take_until, take_while_m_n},
    combinator::{map_res, opt, peek},
    sequence::{delimited, tuple},
    IResult, Parser,
};
use serde::{Deserialize, Serialize};

use super::w3parser::W3StrParser;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct W3Str {
    pub color: bool,
    pub transparency: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub content: String,
}

impl Display for W3Str {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.color {
            write!(
                f,
                "<span style=\"color: rgba({}, {}, {}, 255);\">{}</span>",
                self.red, self.green, self.blue, self.content
            )
        } else {
            write!(f, "{}", self.content)
        }
    }
}

impl W3StrParser for W3Str {
    /// Parse a W3Str from a byte slice.
    ///
    /// Original: `|c00BBGGRRText|r`
    /// Text: `Text`
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, color_prefix) = opt(peek(tag_no_case("|c")))(input)?;
        match color_prefix {
            Some(_) => {
                let (input, content) = delimited(
                    tag_no_case("|c"),
                    take_till1(|c| c == '|' || c == '\0'),
                    tag_no_case("|r"),
                )(input)?;

                let (content, (transparency, red, green, blue)) =
                    tuple((hex_primary, hex_primary, hex_primary, hex_primary))(content)?;

                Ok((
                    input,
                    W3Str {
                        color: true,
                        transparency,
                        red,
                        green,
                        blue,
                        content: content.to_string(),
                    },
                ))
            }
            None => {
                let (input, content) = take_till1(|c| c == '|' || c == '\0')(input)?;
                Ok((
                    input,
                    W3Str {
                        color: false,
                        transparency: 0,
                        red: 0,
                        green: 0,
                        blue: 0,
                        content: content.to_string(),
                    },
                ))
            }
        }
    }
}

/// Parse a 4-character string.
pub fn parse_4char_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, bytes) = take(4usize)(input)?;
    let string = String::from_utf8(bytes.to_vec()).unwrap().to_string();
    Ok((input, string))
}

/// Parse a C-style string.
pub fn parse_cstring(input: &[u8]) -> IResult<&[u8], String> {
    let terminator = "\0";
    let (input, bytes) = take_until(terminator)(input)?;
    let (input, _) = take(1usize)(input)?;
    let string = String::from_utf8(bytes.to_vec()).unwrap().to_string();
    Ok((input, string))
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_w3str() {
        let input = "|c00AABBCCHello, World!|r";
        let expected = W3Str {
            color: true,
            transparency: 0,
            red: 170,
            green: 187,
            blue: 204,
            content: "Hello, World!".to_string(),
        };
        let (_, result) = W3Str::parse(input).unwrap();
        assert_eq!(result, expected);

        let output = format!("{}", result);
        assert_eq!(
            output,
            "<span style=\"color: rgba(170, 187, 204, 255);\">Hello, World!</span>".to_string()
        );
    }

    #[test]
    fn test_parse_w3str_no_color() {
        let input = "Hello, World!";
        let expected = W3Str {
            color: false,
            transparency: 0,
            red: 0,
            green: 0,
            blue: 0,
            content: "Hello, World!".to_string(),
        };
        let (_, result) = W3Str::parse(input).unwrap();
        assert_eq!(result, expected);

        let output = format!("{}", result);
        assert_eq!(output, input);
    }

    #[test]
    fn test_parse_4char_string() {
        let input = b"ABCDEF";
        let expected = "ABCD".to_string();
        let result = parse_4char_string(input);
        assert_eq!(result, Ok((&b"EF"[..], expected)));
    }

    #[test]
    fn test_parse_cstring() {
        let input = b"Hello, World!\0abc";
        let expected = "Hello, World!".to_string();
        let result = parse_cstring(input);
        assert_eq!(result, Ok((&b"abc"[..], expected)));
    }
}
