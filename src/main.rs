use compression::image::{Image, ImgFormat};

fn main() {

    let image = match Image::new(ImgFormat::Qoi, "./src_images/qoi/dice.qoi") {
        Ok(success) => success,
        Err(e) => return eprintln!("{e}"),
    };
    match image.write_file("/home/jon/Pictures/test_building") {
        Err(e) => return eprintln!("{e}"), _ => ()
    }
    //println!("{:?}", image);
}
