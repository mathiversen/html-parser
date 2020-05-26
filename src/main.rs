use html_parser::{Dom, Error, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output format
    #[structopt(short, long, default_value = "json")]
    format: String,

    /// Pretty print
    #[structopt(short, long)]
    pretty_print: bool,
    // TODO: More options
    // Output file, stdout if not present
    // #[structopt(parse(from_os_str))]
    // output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut f = File::open(opt.input)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let dom = Dom::parse(&buffer)?;

    match opt.format.as_str() {
        "json" => {
            if opt.pretty_print {
                println!("{}", dom.to_json_pretty()?);
            } else {
                println!("{}", dom.to_json()?);
            }
            Ok(())
        }
        _ => Err(Error::Cli("Only json is supported".to_string()).into()),
    }
}
