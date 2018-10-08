use std::error::Error;
use std::fs;
use std::io::Write;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kobuta")]
enum Kobuta {
    #[structopt(name = "convert")]
    Convert(Convert),
}

#[derive(StructOpt, Debug)]
struct Convert {
    #[structopt(short = "h", long = "has_headers")]
    has_headers: Option<bool>,

    #[structopt(short = "i", long = "in")]
    input: String,

    #[structopt(short = "o", long = "out")]
    output: String,

    #[structopt(short = "s", long = "schema")]
    schema: String,
}

impl Convert {
    fn run(&self) -> Result<(), Box<Error>> {
        let schema = kobuta::schema::parse(&self.schema)?;

        if self.input.ends_with(".csv") && self.output.ends_with(".kbt") {
            return self.from_csv(&schema, self.has_headers);
        }

        if self.input.ends_with(".kbt") && self.output.ends_with(".csv") {
            if self.has_headers.is_some() {
                // TODO: When we support reading schema from KBT metadata, report error about "needless schema" here
            }
            return self.to_csv(&schema);
        }

        Err("Unsupported conversion!".into())
    }

    fn to_csv(&self, schema: &[kobuta::schema::Column]) -> Result<(), Box<Error>> {
        let kbt = fs::read(&self.input)?;
        let mut output_buff = vec![0; 10 * 1024 * 1024]; // TODO fix this
        let output = kobuta::covert_to_csv(kbt.as_slice(), &mut output_buff, &schema)?;
        fs::write(&self.output, output)?;
        Ok(())
    }

    fn from_csv(
        &self,
        schema: &[kobuta::schema::Column],
        has_headers: Option<bool>,
    ) -> Result<(), Box<Error>> {
        let csv = fs::read(&self.input)?;
        let has_headers = if let Some(has_headers) = self.has_headers {
            has_headers
        } else {
            match kobuta::csv::infer_headers(csv.as_slice()) {
                kobuta::csv::InferResult::AlmostSure(has_headers) => has_headers,
                kobuta::csv::InferResult::NotSure => return Err(
                    "Not sure if the input CSV file has a header row! Please specify --has_headers true/false".into(),
                ),
            }
        };
        let mut output_buff = vec![0; 5 * 1024 * 1024];
        let output = kobuta::csv::to_kbt(csv.as_slice(), &schema, &mut output_buff, has_headers)?;
        fs::write(&self.output, &output)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<Error>> {
    env_logger::init();

    match Kobuta::from_args() {
        Kobuta::Convert(c) => c.run()?,
    }

    Ok(())
}
