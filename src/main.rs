mod convert_256;
mod convert_indexed;

use clap::Parser;

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: PathBuf,

    #[arg(short, long)]
    bin: Option<PathBuf>,

    #[arg(short, long)]
    pal: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let bin_out_path = args
        .bin
        .unwrap_or_else(|| args.path.as_path().with_extension("bin"));
    let pal_out_path = args
        .pal
        .unwrap_or_else(|| args.path.as_path().with_extension("pal.bin"));

    let decoder = png::Decoder::new(File::open(args.path)?);
    let mut reader = decoder.read_info()?;

    let color_type = reader.info().color_type;
    match color_type {
        png::ColorType::Rgb => convert_256::convert(&mut reader, &bin_out_path, &pal_out_path),
        png::ColorType::Indexed => {
            convert_indexed::convert(&mut reader, &bin_out_path, &pal_out_path)
        }
        _ => Err(Error::new(ErrorKind::Other, "Unsupported color type")),
    }
}
