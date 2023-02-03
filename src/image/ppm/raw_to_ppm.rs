use std::error::Error;

use crate::image::Header;

pub fn raw_to_ppm(header: &mut Header, data: &mut Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {

    if &header.format != "P6" {
        header.format = "P6".to_string();
    }
    if String::is_empty(&header.encoding) {
        header.encoding = "255".to_string();
    }
    let mut new_data: Vec<u8> = Vec::new();
    if &header.channels[..] == "RGBA" {
        for (i, n) in data.into_iter().enumerate() {
            if i % 4 != 3 {
                new_data.push(*n);
            }
        }
    } else {
        new_data = data.clone();
    }
    header.contents = header.format.clone() + "\n" +
                      &header.width.to_string() + " " +
                      &header.height.to_string() + "\n" +
                      &header.encoding.to_string() + "\n";
    Ok(new_data)
}
