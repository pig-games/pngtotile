use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use log::debug;
use png::Transformations;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: PathBuf,

    #[arg(short, long)]
    bin: Option<PathBuf>,

    #[arg(short, long)]
    pal: Option<PathBuf>
}
fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let bin_out_path = args.bin.unwrap_or_else(|| args.path.as_path().with_extension("bin"));
    let pal_out_path = args.pal.unwrap_or_else(|| args.path.as_path().with_extension("pal.bin"));

    let mut decoder = png::Decoder::new(File::open(args.path)?);
    let mut trans = Transformations::normalize_to_color8();
    trans.insert(Transformations::from_bits_truncate(0x0002));
    decoder.set_transformations(trans);
    let mut reader = decoder.read_info()?;
    let mut out_file = File::create(bin_out_path)?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    let mut palette: HashMap<(u8, u8, u8), u8> = HashMap::new();
    let mut byte_iter = bytes.iter();
    let mut last_index = 0;
    loop {
        let Some(r) = byte_iter.next() else {
                break;
            };
        let Some(g) = byte_iter.next() else {
                break;
            };
        let Some(b) = byte_iter.next() else {
                break;
            };
        let rgb = (*r, *g, *b);
        if let Some(index) = palette.get(&rgb) {
            out_file.write(&[*index])?;
        } else {
            palette.insert(rgb, last_index);
            out_file.write(&[last_index])?;
            println!("{:?}", rgb);
            last_index += 1;
        }
    }
    let mut palette_file = File::create(pal_out_path)?;
    let mut pal_vec: Vec<(&(u8, u8, u8), &u8)> = palette.iter().collect();
    pal_vec.sort_by(|a, b| a.1.cmp(b.1));
    for ((r,g,b), _) in pal_vec.iter() {
        println!("{},{},{}", r, g, b);
        palette_file.write(&[*b, *g, *r, 255])?;
    }
    Ok(())
}
