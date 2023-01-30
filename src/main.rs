mod image;
use image::Image;
use image::Color;

fn main() -> std::io::Result<()> {
    let mut image = Image::new("./src_images/big_building.ppm").expect("Image build failed");
    image.color_shift(Color::new(50, 0, 50))?;
    image.write_file("/home/jon/Pictures/test_building")?;
    Ok(())
}
