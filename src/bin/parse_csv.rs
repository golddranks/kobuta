use std::error::Error;
use std::fs;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "f", long = "file")]
    filename: String,

    #[structopt(short = "s", long = "schema")]
    schema: String,

    #[structopt(short = "o", long = "out")]
    output: String,
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();

    let opt = Opt::from_args();

    let csv = fs::read(&opt.filename)?;
    let schema = kobuta::schema::parse(&opt.schema)?;
    let mut output = vec![0; 5 * 1024 * 1024];

    kobuta::parse_csv(csv.as_slice(), &schema, &mut output)?;

    fs::write(opt.output, &output)?;

    println!("Done.");

    Ok(())
}