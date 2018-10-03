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

    println!("KBT: {:?}", &kbt[..100]);

    let schema = kobuta::schema::parse(&opt.schema)?;

    let mut output = vec![0; 500000]; // TODO fix this

    kobuta::covert_to_csv(kbt.as_slice(), &mut output, &schema)?;

    fs::write(&opt.output, output)?;

    println!("Done.");

    Ok(())
}