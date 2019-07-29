use termion::screen::AlternateScreen;
use termion::raw::IntoRawMode;

use image::GenericImageView;
use base64;


use std::io::{stdout, BufWriter, Write};
use std::error::Error;
use std::env::args;
use std::fs::File;


fn main() -> Result<(), Box<dyn Error>> {
    let raw_screen = BufWriter::new(stdout()).into_raw_mode()?;
    let _alternate_screen = AlternateScreen::from(raw_screen);

    let image_file = args().skip(1).take(1).collect::<String>();
    let image = image::open(&image_file)?;

    let width = image.width();
    let height = image.height();

    let rgba_image = image.to_rgba();
    let image_bytes = rgba_image.as_flat_samples();

    // Relative paths (like ./image.raw) dont work?
    let mut out_file = File::create("/tmp/image.raw")?;
    out_file.write_all(image_bytes.as_slice())?;

    print!("\x1b_Ga=d\x1b\\");
    println!("\x1b_Gf=32,s={},v={},c={},r={},a=T,t=f;{}\x1b\\",
             width,
             height,
             30,
             20,
             base64::encode("/tmp/image.raw"));
    print!("\x1b_Ga=d\x1b\\");

    Ok(())
}
