# pngtotile

This is small utility for converting a png file to a set of two bin files, one with image data and one with palette data. These bin files can be easily used on the F256Jr/K retro computers for tilemaps and such.

It currently supports both RGB 256 color and indexed color png files. Alpha channels are not supported as of yet.

Transparency for the background is treated as follows:
* indexed color: the first color index will be treated as the transparent background
* RGB 256 color: the palette is generated automatically based on the order of occurence of new colors in the pixel data. So the first color it encounters wil be the first in the palette and therefor will be treated as the transparent background color.

## build and install

The converter is written in Rust and can be easily build using cargo.

# If you don't have Rust installed yet, follow the easy instructions [here](https://rust-lang.org).
# Clone this repository and cd into the pngtotile folder.
# run: cargo install --path .

This should build and install the pngtotile executable and tell you where it is installed into.

If you just want to build and not install just run: cargo build --release. This will perform a release build and store the executable in <your-path>/pngtotile/target/release.

