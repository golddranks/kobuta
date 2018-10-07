use std::mem;

use crate::{
    stripe,
};

// Simple types
pub type Int8 = i8;
pub type Int16 = i16;
pub type Int32 = i32;
pub type Int64 = i64;
pub type Int128 = i128;

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type UInt128 = u128;

pub type Float32 = f32;
pub type Float64 = f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Text16(u16);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Text32(u32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Text64(u64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Data {
    Int8(Int8),
    Int16(Int16),
    Int32(Int32),
    Int64(Int64),
    Int128(Int128),

    UInt8(UInt8),
    UInt16(UInt16),
    UInt32(UInt32),
    UInt64(UInt64),
    UInt128(UInt128),

    Float32(Float32),
    Float64(Float64),

    Text16(Text16),
    Text32(Text32),
    Text64(Text64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataType {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,

    Float32,
    Float64,

    Text16,
    Text32,
    Text64,
}

impl DataType {
    pub fn size(&self) -> usize {
        match self {
            DataType::Int8 => mem::size_of::<Int8>(),
            DataType::Int16 => mem::size_of::<Int16>(),
            DataType::Int32 => mem::size_of::<Int32>(),
            DataType::Int64 => mem::size_of::<Int64>(),
            DataType::Int128 => mem::size_of::<Int128>(),

            DataType::UInt8 => mem::size_of::<UInt8>(),
            DataType::UInt16 => mem::size_of::<UInt16>(),
            DataType::UInt32 => mem::size_of::<UInt32>(),
            DataType::UInt64 => mem::size_of::<UInt64>(),
            DataType::UInt128 => mem::size_of::<UInt128>(),

            DataType::Float32 => mem::size_of::<Float32>(),
            DataType::Float64 => mem::size_of::<Float64>(),

            DataType::Text16 => mem::size_of::<Text16>(),
            DataType::Text32 => mem::size_of::<Text32>(),
            DataType::Text64 => mem::size_of::<Text64>(),
        }
    }

    pub fn stripe<'b>(&self, bytes: &'b [u8]) -> stripe::Ref<'b> {
        use crate::stripe::{Ref, from_bytes};
        match self {
            DataType::Int8 => Ref::Int8(from_bytes(bytes)),
            DataType::Int16 => Ref::Int16(from_bytes(bytes)),
            DataType::Int32 => Ref::Int32(from_bytes(bytes)),
            DataType::Int64 => Ref::Int64(from_bytes(bytes)),
            DataType::Int128 => Ref::Int128(from_bytes(bytes)),

            DataType::UInt8 => Ref::UInt8(from_bytes(bytes)),
            DataType::UInt16 => Ref::UInt16(from_bytes(bytes)),
            DataType::UInt32 => Ref::UInt32(from_bytes(bytes)),
            DataType::UInt64 => Ref::UInt64(from_bytes(bytes)),
            DataType::UInt128 => Ref::UInt128(from_bytes(bytes)),

            DataType::Float32 => Ref::Float32(from_bytes(bytes)),
            DataType::Float64 => Ref::Float64(from_bytes(bytes)),

            DataType::Text16 => Ref::Text16(from_bytes(bytes)),
            DataType::Text32 => Ref::Text32(from_bytes(bytes)),
            DataType::Text64 => Ref::Text64(from_bytes(bytes)),
        }
    }

    pub fn stripe_mut<'b>(&self, bytes: &'b mut [u8]) -> stripe::Mut<'b> {
        use crate::stripe::{Mut, from_bytes_mut};
        match self {
            DataType::Int8 => Mut::Int8(from_bytes_mut(bytes)),
            DataType::Int16 => Mut::Int16(from_bytes_mut(bytes)),
            DataType::Int32 => Mut::Int32(from_bytes_mut(bytes)),
            DataType::Int64 => Mut::Int64(from_bytes_mut(bytes)),
            DataType::Int128 => Mut::Int128(from_bytes_mut(bytes)),

            DataType::UInt8 => Mut::UInt8(from_bytes_mut(bytes)),
            DataType::UInt16 => Mut::UInt16(from_bytes_mut(bytes)),
            DataType::UInt32 => Mut::UInt32(from_bytes_mut(bytes)),
            DataType::UInt64 => Mut::UInt64(from_bytes_mut(bytes)),
            DataType::UInt128 => Mut::UInt128(from_bytes_mut(bytes)),

            DataType::Float32 => Mut::Float32(from_bytes_mut(bytes)),
            DataType::Float64 => Mut::Float64(from_bytes_mut(bytes)),

            DataType::Text16 => Mut::Text16(from_bytes_mut(bytes)),
            DataType::Text32 => Mut::Text32(from_bytes_mut(bytes)),
            DataType::Text64 => Mut::Text64(from_bytes_mut(bytes)),
        }
    }

}