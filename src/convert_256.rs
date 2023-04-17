use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use png::Reader;

pub(crate) fn convert(
    reader: &mut Reader<File>,
    bin_out_path: &PathBuf,
    pal_out_path: &PathBuf,
) -> std::io::Result<()> {
    println!("256");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    let mut out_file = File::create(bin_out_path)?;
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
            out_file.write_all(&[*index])?;
        } else {
            palette.insert(rgb, last_index);
            out_file.write_all(&[last_index])?;
            println!("{}: {:?}", last_index, rgb);
            last_index += 1;
        }
    }
    let mut palette_file = File::create(pal_out_path)?;
    let mut pal_vec: Vec<(&(u8, u8, u8), &u8)> = palette.iter().collect();
    pal_vec.sort_by(|a, b| a.1.cmp(b.1));
    for ((r, g, b), i) in pal_vec.iter() {
        println!("{}: ({},{},{})", i, r, g, b);
        palette_file.write_all(&[*b, *g, *r, 255])?;
    }
    Ok(())
}
