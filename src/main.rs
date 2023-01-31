mod image;
use image::{Image, Color};

fn main() -> std::io::Result<()> {

        let mut image = Image::new("./src_images/big_tree.ppm").expect("Image build failed");
        image.color_shift(Color::new(0, 100, 50))?;
        image.write_file("/home/jon/Pictures/test_tree")?;

    Ok(())
}
