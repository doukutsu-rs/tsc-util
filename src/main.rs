mod tsc;

use std::path::PathBuf;

use clap::Parser;

use crate::tsc::TSC;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the dir containing files to be processed
    input: String,

    #[arg(short, long, default_value_t = String::from("output"))]
    output: String,

    /// If isn't set, then the files will be decoded
    #[arg(short, long, default_value_t = false)]
    encode: bool,

    #[arg(short, long, default_value_t = false)]
    decode: bool,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut processed_files = 0;
    let tsc_handler = TSC::new(args.verbose);

    if args.encode {
        processed_files = tsc_handler.encode(
            PathBuf::from(args.input.as_str()).into_boxed_path(),
            PathBuf::from(args.output.as_str()).into_boxed_path(),
        )?;

        println!("{} files encoded", processed_files);
    } else {
        processed_files = tsc_handler.decode(
            PathBuf::from(args.input.as_str()).into_boxed_path(),
            PathBuf::from(args.output.as_str()).into_boxed_path(),
        )?;

        println!("{} files decoded", processed_files);
    }

    Ok(())
}
