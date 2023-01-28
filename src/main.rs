mod read_image;

use read_image::Image;

fn main() {
    let image = Image::build("./test_images/artificial.ppm").expect("Image build failed");
    image.print();
}
