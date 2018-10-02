use std::error::Error;
use std::fmt;
use std::mem;
use std::io::Read;
use std::slice;
use log::debug;

pub mod schema; // TODO check if we need this mod clause in Rust 2018

use crate::schema::Parse;

const STRIPE_SIZE: u16 = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KbtError;

impl Error for KbtError {
    fn description(&self) -> &str {
        "Kobuta error"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for KbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kobuta error")
    }
}

/// Returns a stripe size in bytes
fn stripe_size(col: &schema::Column) -> u16 {
    col.dtype.size() * STRIPE_SIZE
}

/// Returns a strip offset in bytes
fn calc_stripe_offset(schema: &[schema::Column], col: u32) -> u32 {
    schema[..col as usize].iter()
        .fold(0, |size, col| size + stripe_size(col) as u32)
}

/// Returns a sized partition size in bytes
fn calc_sized_partition_size(schema: &[schema::Column]) -> u32 {
    schema.iter()
        .fold(0, |size, col| size + stripe_size(col) as u32)
}

fn calc_val_offset(col: &schema::Column, row: u32) -> u32 {
    col.dtype.size() as u32 * row
}

enum Stripe<'a> {
    Int32(&'a mut [i32; STRIPE_SIZE as usize]),
    Float32(&'a mut [f32; STRIPE_SIZE as usize]),
}

fn bytes_to_stripe<T>(bytes: &mut [u8]) -> &mut [T; STRIPE_SIZE as usize] {
    assert_eq!(bytes.len(), mem::size_of::<[T; STRIPE_SIZE as usize]>());
    // TODO alignment needs to be checked

    let ptr = bytes.as_mut_ptr() as *mut [T; STRIPE_SIZE as usize];
    unsafe { &mut *ptr }
}

fn partition<'out, 's>(mut block: &'out mut [u8], schema: &'s[schema::Column]) -> Vec<Stripe<'out>> {
    if block.len() < calc_sized_partition_size(schema) as usize {
        panic!("The block buffer is too small!"); // TODO maybe do proper error handling
    }

    let mut stripes = Vec::with_capacity(schema.len());

    for (col_i, col) in schema.iter().enumerate() {
        let (bytes, unpartitioned) = block.split_at_mut(stripe_size(col) as usize);

        block = unpartitioned;

        let stripe =
            match col.dtype {
                schema::DataType::Float32 => Stripe::Float32(bytes_to_stripe(bytes)),
                schema::DataType::Int32 => Stripe::Int32(bytes_to_stripe(bytes)),

        };

        stripes.push(stripe);
    }

    stripes
}

/// This is a baseline implementation of a write function that is likely to
/// perform really badly.
fn write_val<T>(block: &mut [u8], schema: &[schema::Column], col: u32, row: u32, val: T) {
    let block_ptr = block.as_mut_ptr() as *mut T;

    let offset = calc_stripe_offset(schema, col)
        + calc_val_offset(&schema[col as usize], row);


    unsafe {
        let val_ptr = block_ptr.offset(offset as isize);
        *val_ptr = val;
    };
}

pub fn parse_csv(reader: impl Read, schema: &[schema::Column], output: &mut [u8]) -> Result<(), Box<Error>> {

    let mut csv_reader = csv::Reader::from_reader(reader);

    let block_size = calc_sized_partition_size(schema) as usize;

    for block in output.chunks_mut(block_size) {

        let mut stripes = partition(block, schema);

        for (rownum, rec) in csv_reader.records().enumerate().take(STRIPE_SIZE as usize) {

            let rec = rec?;

            for (i, col) in schema.into_iter().enumerate() {
                let field = &rec[i];

                match stripes[i] {
                    Stripe::Float32(ref mut stripe) => stripe[rownum] = f32::parse(field)?,
                    Stripe::Int32(ref mut stripe) => stripe[rownum] = i32::parse(field)?,
                }
            }
        }
    }

    Ok(())
}