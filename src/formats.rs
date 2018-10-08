use crate::{errors::KbtError, types::*};

pub trait Fmt {
    fn parse_int8(val: &[u8]) -> Result<Int8, KbtError>;
    fn parse_int16(val: &[u8]) -> Result<Int16, KbtError>;
    fn parse_int32(val: &[u8]) -> Result<Int32, KbtError>;
    fn parse_float32(val: &[u8]) -> Result<Float32, KbtError>;

    fn print_int8(val: Int8, output: &mut [u8]) -> Result<usize, KbtError>;
    fn print_int16(val: Int16, output: &mut [u8]) -> Result<usize, KbtError>;
    fn print_int32(val: Int32, output: &mut [u8]) -> Result<usize, KbtError>;
    fn print_float32(val: Float32, output: &mut [u8]) -> Result<usize, KbtError>;
}
