use std::error::Error;
use std::fs;
use std::io::Write;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
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

    //let kbt = fs::File::open(&opt.input)?;
    let kbt = fs::read(&opt.input)?;

    let schema = kobuta::schema::parse(&opt.schema)?;

    let mut output_buff = vec![0; 10 * 1024 * 1024]; // TODO fix this

    let output = kobuta::covert_to_csv(kbt.as_slice(), &mut output_buff, &schema)?;

    fs::write(&opt.output, output)?;

    println!("Done.");

    Ok(())
}
