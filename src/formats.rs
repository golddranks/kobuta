use crate::{
    errors::KbtError,
};

pub trait Fmt {
    fn parse_int8(val: &[u8]) -> Result<i8, KbtError>;
    fn parse_int16(val: &[u8]) -> Result<i16, KbtError>;

    fn print_int8(val: i8, output: &mut [u8]) -> Result<usize, KbtError>;
    fn print_int16(val: i16, output: &mut [u8]) -> Result<usize, KbtError>;
}