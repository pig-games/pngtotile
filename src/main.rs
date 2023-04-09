use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

use png::Transformations;

fn main() -> std::io::Result<()> {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let mut decoder = png::Decoder::new(File::open("test.png").unwrap());
    let mut trans = Transformations::normalize_to_color8();
    trans.insert(Transformations::from_bits_truncate(0x0002));
    decoder.set_transformations(trans);
    let mut reader = decoder.read_info().unwrap();
    eprintln!("png info: {:?}", reader.info());
    // Allocate the output buffer.
    // let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    // let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    // let bytes = &buf[..info.buffer_size()];
    let mut out_file = File::create("out.bin").unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    let mut palette: HashMap<(u8, u8, u8), u8> = HashMap::new();
    let mut byte_iter = bytes.iter();
    let mut last_index = 1;
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
            out_file.write(&[*index]).unwrap();
        } else {
            last_index += 1;
            palette.insert(rgb, last_index);
            out_file.write(&[last_index]).unwrap();
            eprintln!("{:?}", rgb);
        }
    }
    Ok(())
    // write bytes to out_file
}
