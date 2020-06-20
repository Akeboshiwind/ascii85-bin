use std::fs::File;
use std::io::{self, Read, Write};
use std::str;

use anyhow::{Context, Result};
use gumdrop::Options;

#[derive(Debug, Options)]
struct Arguments {
    #[options(help = "print this help message")]
    help: bool,

    #[options(help = "decode the input message")]
    decode: bool,

    #[options(help = "input file", default = "-")]
    input: String,

    #[options(help = "output file", default = "-")]
    output: String,
}

fn main() -> Result<()> {
    let opts = Arguments::parse_args_default_or_exit();

    // Load the input
    let mut input = String::new();
    if opts.input == "-" {
        // Read from stdin
        io::stdin()
            .read_to_string(&mut input)
            .context("Unable to read from stdin")?;
    } else {
        // Read from file
        let filename = opts.input;
        let mut file =
            File::open(&filename).context(format!("Failed to open input file {}", &filename))?;

        file.read_to_string(&mut input)
            .context(format!("Unable to read from file {}", &filename))?;
    }

    // Decode or encode the input
    let out = if opts.decode {
        let out = ascii85::decode(&input).unwrap();

        str::from_utf8(&out)?.to_owned()
    } else {
        ascii85::encode(&input.into_bytes())
    };

    // Output
    if opts.output == "-" {
        // Output to Stdin
        write!(io::stdout(), "{}", out).context("Failed to write to stdout")?;
    } else {
        // Output to file
        let filename = opts.output;
        let mut file =
            File::create(&filename).context(format!("Failed to open output file {}", &filename))?;

        write!(file, "{}", out).context(format!("Failed to write out output file {}", &filename))?
    }

    Ok(())
}
