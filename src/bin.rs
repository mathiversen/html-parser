use html_parser::prelude::{Parser, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut file = File::open(opt.input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Parser::parse(&contents, true)?;
    Ok(())
}
