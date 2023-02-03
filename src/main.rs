use compression::image::{Image, ImgFormat};

fn main() {

//    let mut image = match Image::new(ImgFormat::Qoi, "./src_images/qoi/kodim23.qoi") {
//        Ok(success) => success,
//        Err(e) => return eprintln!("{e}"),
//    };
    let mut image = match Image::new(ImgFormat::Qoi, "./src_images/qoi/dice.qoi") {
        Ok(success) => success,
        Err(e) => return eprintln!("{e}"),
    };
    image.convert(ImgFormat::Ppm).expect("Conversion failed");
    match image.write_file("/home/jon/Pictures/testcard") {
        Err(e) => return eprintln!("{e}"), _ => ()
    }
}
