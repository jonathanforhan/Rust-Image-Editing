use std::error::Error;

use image_edit::image::{Image, ImgFormat};

fn main() -> Result<(), Box<dyn Error>> {

    let mut image = Image::new("./src_images/qoi/testcard.qoi")?;
    image.convert(ImgFormat::Ppm)?.color_shift((0, 0, 100))?.write_file("/home/jon/Pictures/testcard")?;
    Ok(())
}
