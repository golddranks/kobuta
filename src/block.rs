use crate::{
    schema,
    stripe,
};

/// Returns a sized partition size in bytes
pub fn calc_sized_partition_size(schema: &[schema::Column]) -> usize {
    schema
        .iter()
        .fold(0, |size, col| size + stripe::size(&col.dtype))
}

fn calc_val_offset(col: &schema::Column, row: u32) -> u32 {
    col.dtype.size() as u32 * row
}

pub fn partition<'b>(
    mut block: &'b [u8],
    schema: &[schema::Column],
) -> Vec<stripe::Ref<'b>> {
    let mut stripes = Vec::with_capacity(schema.len());

    for (col_i, col) in schema.iter().enumerate() {
        let (bytes, unpartitioned) = block.split_at(stripe::size(&col.dtype) as usize);

        block = unpartitioned;

        let stripe = col.dtype.stripe(bytes);
        stripes.push(stripe);
    }

    stripes
}

pub fn partition_mut<'b>(
    mut block: &'b mut [u8],
    schema: &[schema::Column],
) -> Vec<stripe::Mut<'b>> {
    let mut stripes = Vec::with_capacity(schema.len());

    for (col_i, col) in schema.iter().enumerate() {
        let (bytes, unpartitioned) = block.split_at_mut(stripe::size(&col.dtype) as usize);

        block = unpartitioned;

        let stripe = col.dtype.stripe_mut(bytes);
        stripes.push(stripe);
    }

    stripes
}
