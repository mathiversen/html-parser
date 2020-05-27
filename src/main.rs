use html_parser::{Dom, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A simple and general purpose html/xhtml parser.")]
struct Opt {
    #[structopt(short, long)]
    /// Pretty-print the output
    pretty_print: bool,

    /// Html to parse
    input: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let dom = Dom::parse(&opt.input)?;

    if opt.pretty_print {
        println!("{}", dom.to_json_pretty()?);
    } else {
        println!("{}", dom.to_json()?);
    }
    Ok(())
}
