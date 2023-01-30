mod image;
use image::Image;
use image::Color;

fn main() -> std::io::Result<()> {
    let mut image = Image::new("./src_images/big_building.ppm").expect("Image build failed");
    image.color_shift(Color::new(0, 120, 0))?;
    image.write_file("/home/jon/Pictures/test_building")?;
    Ok(())
}
