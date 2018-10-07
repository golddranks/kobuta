use std::error::Error;
use std::fs;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "h", long = "has_headers")]
    has_headers: bool,

    #[structopt(short = "i", long = "in")]
    input: String,

    #[structopt(short = "o", long = "out")]
    output: String,

    #[structopt(short = "s", long = "schema")]
    schema: String,
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();

    let opt = Opt::from_args();

    let csv = fs::read(&opt.input)?;
    let schema = kobuta::schema::parse(&opt.schema)?;
    let mut output_buff = vec![0; 5 * 1024 * 1024];

    let output = kobuta::parse_csv(csv.as_slice(), &schema, &mut output_buff, opt.has_headers)?;

    fs::write(opt.output, &output)?;

    println!("Done.");

    Ok(())
}