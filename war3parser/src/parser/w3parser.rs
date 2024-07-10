use nom::IResult;

pub trait W3BytesParser {
    fn parse(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

pub trait W3StrParser {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

pub fn get_bit_from_u32(value: u32, bit: u32) -> bool {
    (value & (1 << bit)) != 0
}
