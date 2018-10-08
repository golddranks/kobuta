use log::{debug, trace};
use std::error::Error;
use std::io::{ErrorKind as IoErrorKind, Read, Write};
use std::mem;
use std::slice;

mod block;
pub mod csv;
pub mod errors;
mod formats;
pub mod schema;
mod stripe;
pub mod types;

use crate::errors::KbtError;

// TODO unused at the moment
/*
/// This is a baseline implementation of a write function that is likely to
/// perform really badly.
fn write_val<T>(block: &mut [u8], schema: &[schema::Column], col: u32, row: u32, val: T) {
    let block_ptr = block.as_mut_ptr() as *mut T;

    let offset = block::calc_stripe_offset(schema, col) + calc_val_offset(&schema[col as usize], row);

    unsafe {
        let val_ptr = block_ptr.offset(offset as isize);
        *val_ptr = val;
    };
}
*/

pub fn covert_to_csv<'o>(
    mut input: impl Read,
    output: &'o mut [u8],
    schema: &[schema::Column],
) -> Result<&'o [u8], KbtError> {
    let block_size = block::calc_sized_partition_size(schema);

    let mut block_buff = vec![0; block_size as usize];

    let mut output_consumed = 0;

    let mut output_left = &mut *output;

    loop {
        match &input.read_exact(&mut block_buff) {
            Ok(()) => (),
            Err(err) if err.kind() == IoErrorKind::UnexpectedEof => {
                debug!("UnexpectedEof");
                break;
            }
            Err(err) => {
                debug!("read_exact: {:?}", err);
                return Err(KbtError);
            }
        }
        let (output, output_left_temp) = csv::from_block(&mut block_buff, output_left, schema)?;

        output_left = output_left_temp;

        output_consumed += output.len()
    }

    Ok(&output[..output_consumed])
}
