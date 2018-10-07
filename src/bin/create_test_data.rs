use std::{
    error::Error,
    io::{Error as IoError, Write},
};

use rand::Rng;
use structopt::StructOpt;

use kobuta::{
    schema::Column,
    types::DataType,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "s", long = "schema")]
    schema: String,

    #[structopt(short = "n", long = "records")]
    records: usize,
}

fn prop(prop: f64) -> bool {
    rand::thread_rng().gen_bool(prop)
}

fn output_csv_field(col: &Column, output: &mut impl Write) -> Result<(), IoError> {
    if col.nullable && prop(0.7) {
        output.write(b"null")?;
        return Ok(());
    }
    let mut rng = rand::thread_rng();
    use self::DataType::*;
    match col.dtype {
        Float32 => dtoa::write(output, rng.gen::<f32>())?,
        Int32 => itoa::write(output, rng.gen::<i32>())?,
        _ => unimplemented!(), // TODO
    };
    Ok(())
}

fn output_csv_row(schema: &[Column], output: &mut impl Write) -> Result<(), IoError> {
    let last_col_idx = schema.len() - 1;
    for col in &schema[0..last_col_idx] {
        output_csv_field(col, output)?;
        output.write(b",")?;
    }
    output_csv_field(&schema[last_col_idx], output)?;
    output.write(b"\n")?;

    Ok(())
}

fn output_csv(records: usize, schema: &[Column], output: &mut impl Write) -> Result<(), IoError> {
    for rownum in 0..records {
        output_csv_row(schema, output)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();
    let opt = Opt::from_args();

    let schema = kobuta::schema::parse(&opt.schema)?;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    output_csv(opt.records, &schema, &mut stdout)?;

    Ok(())
}
