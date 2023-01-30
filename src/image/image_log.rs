use super::ImageHeader;

pub fn image_log(image_header: &ImageHeader, data: &Vec<u8>) {
    println!("Format: {}", image_header.format);
    println!("Width: {}px", image_header.width);
    println!("Height: {}px", image_header.height);
    println!("Encoding: {}", image_header.encoding);
    println!("File size: {} bytes", data.len() + image_header.size);
}
