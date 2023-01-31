use super::ImageHeader;

pub fn image_log(image_header: &ImageHeader, data_len: usize) {
    println!("Format: {}", image_header.format);
    println!("Width: {}px", image_header.width);
    println!("Height: {}px", image_header.height);
    println!("Encoding: {}", image_header.encoding);
    println!("File size: {} bytes", data_len + image_header.size);
}
