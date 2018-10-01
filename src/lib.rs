use std::error::Error;
use std::fmt;
use std::io::Read;
use log::debug;

pub mod schema;

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

/// This is a baseline implementation of a write function that is likely to
/// perform really badly.
fn write_i32(block: &mut [u8], schema: &[schema::Column], col: i32, val: i32) {

}

pub fn parse_csv(reader: impl Read, schema: &[schema::Column]) -> Result<(), Box<Error>> {

    let mut csv_reader = csv::Reader::from_reader(reader);

    for rec in csv_reader.records() {

        let rec = rec?;

        for (i, col) in schema.into_iter().enumerate() {
            let data = col.parse_data(&rec[i])?;

        }
    }

    Ok(())
}