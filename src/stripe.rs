use std::mem::size_of;

use crate::{errors::KbtError, formats::Fmt, schema, types, types::*};

pub const STRIPE_ALIGNMENT: usize = 64;
pub const STRIPE_SIZE: usize = 64;

/// Returns a stripe size in bytes
pub fn size(dtype: &types::DataType) -> usize {
    dtype.size() * STRIPE_SIZE
}

/// Returns a strip offset in bytes in a given schema
fn calc_offset(schema: &[schema::Column], col: u32) -> u32 {
    schema[..col as usize]
        .iter()
        .fold(0, |acc_offset, col| acc_offset + size(&col.dtype) as u32)
}

fn is_aligned_at(bytes: &[u8], alignment: usize) -> bool {
    ((bytes.as_ptr() as usize) & (alignment-1)) == 0
}

fn slice_ok<T>(bytes: &[u8]) -> bool {
    bytes.len() == size_of::<Stripe<T>>()
        && is_aligned_at(bytes, STRIPE_ALIGNMENT)
}

pub fn from_bytes_mut<T>(bytes: &mut [u8]) -> &mut Stripe<T> {
    assert!(slice_ok::<T>(bytes));

    let ptr = bytes.as_mut_ptr() as *mut Stripe<T>;
    unsafe { &mut *ptr }
}

pub fn from_bytes<T>(bytes: &[u8]) -> &Stripe<T> {
    assert!(slice_ok::<T>(bytes));

    let ptr = bytes.as_ptr() as *const Stripe<T>;
    unsafe { &*ptr }
}

#[repr(align(64))]
pub struct Stripe<T>([T; STRIPE_SIZE]);

impl<T> Stripe<T> {
    fn arr(&self) -> &[T; STRIPE_SIZE] {
        &self.0
    }
}

impl<T> Stripe<T> {
    fn arr_mut(&mut self) -> &mut [T; STRIPE_SIZE] {
        &mut self.0
    }
}

pub enum Ref<'a> {
    Int8(&'a Stripe<Int8>),
    Int16(&'a Stripe<Int16>),
    Int32(&'a Stripe<Int32>),
    Int64(&'a Stripe<Int64>),
    Int128(&'a Stripe<Int128>),

    UInt8(&'a Stripe<Int8>),
    UInt16(&'a Stripe<Int16>),
    UInt32(&'a Stripe<Int32>),
    UInt64(&'a Stripe<Int64>),
    UInt128(&'a Stripe<Int128>),

    Float32(&'a Stripe<Float32>),
    Float64(&'a Stripe<Float64>),

    Text16(&'a Stripe<Text16>),
    Text32(&'a Stripe<Text32>),
    Text64(&'a Stripe<Text64>),
}

impl<'a> Ref<'a> {
    pub fn read_one<F: Fmt>(&self, rownum: usize, output: &mut [u8]) -> Result<usize, KbtError> {
        use self::Ref;
        Ok(match self {
            Ref::Int8(stripe) => F::print_int8(stripe.arr()[rownum], output)?,
            Ref::Int16(stripe) => F::print_int16(stripe.arr()[rownum], output)?,
            Ref::Int32(stripe) => F::print_int32(stripe.arr()[rownum], output)?,
            Ref::Float32(stripe) => F::print_float32(stripe.arr()[rownum], output)?,
            _ => unimplemented!(),
        })
    }
}
pub enum Mut<'a> {
    Int8(&'a mut Stripe<Int8>),
    Int16(&'a mut Stripe<Int16>),
    Int32(&'a mut Stripe<Int32>),
    Int64(&'a mut Stripe<Int64>),
    Int128(&'a mut Stripe<Int128>),

    UInt8(&'a mut Stripe<Int8>),
    UInt16(&'a mut Stripe<Int16>),
    UInt32(&'a mut Stripe<Int32>),
    UInt64(&'a mut Stripe<Int64>),
    UInt128(&'a mut Stripe<Int128>),

    Float32(&'a mut Stripe<Float32>),
    Float64(&'a mut Stripe<Float64>),

    Text16(&'a mut Stripe<Text16>),
    Text32(&'a mut Stripe<Text32>),
    Text64(&'a mut Stripe<Text64>),
}

impl<'a> Mut<'a> {
    pub fn write_one<F: Fmt>(&mut self, rownum: usize, val: &[u8]) -> Result<(), KbtError> {
        use self::Mut;
        match self {
            Mut::Int8(stripe) => stripe.arr_mut()[rownum] = F::parse_int8(val)?,
            Mut::Int16(stripe) => stripe.arr_mut()[rownum] = F::parse_int16(val)?,
            Mut::Int32(stripe) => stripe.arr_mut()[rownum] = F::parse_int32(val)?,
            Mut::Float32(stripe) => stripe.arr_mut()[rownum] = F::parse_float32(val)?,
            _ => unimplemented!(),
        };

        Ok(())
    }
}
