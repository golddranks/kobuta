extern crate rand;
extern crate structopt;
extern crate kobuta;
extern crate env_logger;

use structopt::StructOpt;

use kobuta::schema::Column;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "s", long = "schema")]
    schema: String,

    #[structopt(short = "n", long = "records")]
    records: u32,
}




fn output_csv(records: usize, schema: &[Column]) {

}

fn main() {
    env_logger::init();
    let opt = Opt::from_args();

    let schema = kobuta::schema::parse(&opt.schema);

    eprintln!("{:?}", schema);
}