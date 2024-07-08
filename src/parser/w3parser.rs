use nom::{
    bytes::complete::{take, take_until},
    IResult,
};

pub trait W3Parser {
    fn parse(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

pub fn parse_4char_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, bytes) = take(4usize)(input)?;
    let string = String::from_utf8(bytes.to_vec()).unwrap().to_string();
    Ok((input, string))
}

pub fn parse_cstring(input: &[u8]) -> IResult<&[u8], String> {
    let terminator = "\0";
    let (input, bytes) = take_until(terminator)(input)?;
    let (input, _) = take(1usize)(input)?;
    let string = String::from_utf8(bytes.to_vec()).unwrap().to_string();
    Ok((input, string))
}

pub fn get_bit_from_u32(value: u32, bit: u32) -> bool {
    (value & (1 << bit)) != 0
}
