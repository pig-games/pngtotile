use std::{fs::File, io::Write, path::PathBuf};

use png::Reader;

pub(crate) fn convert(
    reader: &mut Reader<File>,
    bin_out_path: &PathBuf,
    pal_out_path: &PathBuf,
) -> std::io::Result<()> {
    println!("indexed");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    let Some(ref palette) = reader.info().palette else {
        return Ok(())
    };
    let mut component_index = 0;
    let mut palette_file = File::create(pal_out_path)?;
    loop {
        let r = palette[component_index];
        component_index += 1;
        let g = palette[component_index];
        component_index += 1;
        let b = palette[component_index];
        component_index += 1;
        palette_file.write_all(&[b, g, r, 255])?;
        if component_index >= palette.len() {
            break;
        }
    }
    let mut out_file = File::create(bin_out_path)?;
    let bytes = &buf[..info.buffer_size()];
    out_file.write_all(bytes)
}
