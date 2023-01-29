mod read_image;

use read_image::Image;

fn main() -> std::io::Result<()> {
    let mut image = Image::build("./src_images/artificial.ppm").expect("Image build failed");
    image.color_shift("red", "./dst_images/red_shifted", 80)?;
    image.color_shift("green", "./dst_images/green_shifted", 50)?;
    image.color_shift("blue", "./dst_images/blue_shifted", 100)?;
    Ok(())
}
