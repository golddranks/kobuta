use std::{
    io::Read,
    str::{from_utf8, FromStr},
};

use crate::{block, errors::KbtError, formats::Fmt, schema, stripe::STRIPE_SIZE, types::*};

struct Csv;

impl Fmt for Csv {
    fn parse_int8(val: &[u8]) -> Result<Int8, KbtError> {
        unimplemented!()
    }

    fn parse_int16(val: &[u8]) -> Result<Int16, KbtError> {
        unimplemented!()
    }

    fn parse_int32(val: &[u8]) -> Result<Int32, KbtError> {
        i32::from_str_radix(from_utf8(val).map_err(|_| KbtError)?, 10).map_err(|_| KbtError)
    }

    fn parse_float32(val: &[u8]) -> Result<Float32, KbtError> {
        f32::from_str(from_utf8(val).map_err(|_| KbtError)?).map_err(|_| KbtError)
    }

    fn print_int8(val: Int8, output: &mut [u8]) -> Result<usize, KbtError> {
        unimplemented!()
    }

    fn print_int16(val: Int16, output: &mut [u8]) -> Result<usize, KbtError> {
        unimplemented!()
    }

    fn print_int32(val: Int32, output: &mut [u8]) -> Result<usize, KbtError> {
        itoa::write(&mut *output, val).map_err(|_| KbtError)
    }

    fn print_float32(val: Float32, output: &mut [u8]) -> Result<usize, KbtError> {
        dtoa::write(&mut *output, val).map_err(|_| KbtError)
    }
}

pub fn to_kbt<'o>(
    reader: impl Read,
    schema: &[schema::Column],
    output: &'o mut [u8],
    has_headers: bool,
) -> Result<&'o [u8], KbtError> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_reader(reader);
    let mut records = &mut csv_reader.records().peekable();

    let block_size = block::calc_sized_partition_size(schema) as usize;

    let mut blocknum = 0;

    for block in output.chunks_mut(block_size) {
        let mut stripes = block::partition_mut(block, schema);

        for (rownum, rec) in records.enumerate().take(STRIPE_SIZE as usize) {
            let rec = rec.map_err(|_| KbtError)?; // TODO error handling

            for (i, stripe) in stripes.iter_mut().enumerate() {
                stripe.write_one::<Csv>(rownum, rec[i].as_bytes())?;
            }
        }

        blocknum += 1;

        if records.peek().is_none() {
            break;
        }
    }

    Ok(&output[..blocknum * block_size])
}

pub fn from_block<'o>(
    block_buff: &[u8],
    output: &'o mut [u8],
    schema: &[schema::Column],
) -> Result<(&'o [u8], &'o mut [u8]), KbtError> {
    let mut stripes = block::partition(block_buff, schema);

    let mut block_output = &mut *output;

    let mut output_consumed = 0;

    for rownum in 0..STRIPE_SIZE {
        let (last_stripe, stripes) = if let Some(stripes) = stripes.split_last_mut() {
            stripes
        } else {
            break;
        };

        for (i, stripe) in stripes.iter().enumerate() {
            let bytes_written = stripe.read_one::<Csv>(rownum, block_output)?;

            output_consumed += bytes_written;
            block_output = &mut block_output[bytes_written..];

            block_output[0] = b',';
            output_consumed += 1;
            block_output = &mut block_output[1..];
        }

        let bytes_written = last_stripe.read_one::<Csv>(rownum, block_output)?;

        output_consumed += bytes_written;
        block_output = &mut block_output[bytes_written..];

        block_output[0] = b'\n';
        output_consumed += 1;
        block_output = &mut block_output[1..];
    }

    let (output, output_left) = output.split_at_mut(output_consumed);

    Ok((&*output, output_left))
}

pub enum InferResult<T> {
    AlmostSure(T),
    NotSure,
}

pub fn infer_headers(input: &[u8]) -> InferResult<bool> {
    InferResult::NotSure // TODO implement inference
}
