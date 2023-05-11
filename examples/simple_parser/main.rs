use clap::Parser;
use html_parser::{Dom, Result};
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

#[derive(Debug, Parser)]
/// A simple and general purpose html/xhtml parser.
struct Opt {
    #[arg(short, long)]
    /// Pretty-print the output.
    pretty_print: bool,

    #[arg(short, long)]
    /// Debug the parser, this will print errors to the console.
    debug: bool,

    /// Path to the file, or stdin (piped content).
    ///
    /// This argument can either be a path to the html-file that you would like to parse or the
    /// result of stdin. Note: Content over stdin needs to be finite, for now, as it is collected
    /// into a string and then processed by the parser.
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    let mut content = String::with_capacity(100_000);

    // If input is provided then use that as a path
    if let Some(path) = opt.input {
        let mut file = File::open(path)?;
        file.read_to_string(&mut content)?;

    // Else read from stdin, this enables piping
    // ex: `cat index.html | html_parser`
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut content)?;
    };

    let dom = Dom::parse(&content)?;

    if opt.debug {
        for error in &dom.errors {
            println!("# {}", error);
        }
    }

    if opt.pretty_print {
        println!("{}", dom.to_json_pretty()?);
    } else {
        println!("{}", dom.to_json()?);
    }

    Ok(())
}
