use std::{
    fs::File,
    io::{BufReader, Seek, Read},
    error::Error, process,
    str,
};

use crate::image::Header;

pub fn qoi_header(mut file: &File) -> Result<Header, Box<dyn Error>> {
    let mut reader = BufReader::new(file);
    let mut header_buf = vec![0u8; 14];

    let bytes_to_u32 = |b: &[u8]| -> u32 {
        ((b[0] as u32) << 24) +
        ((b[1] as u32) << 16) +
        ((b[2] as u32) <<  8) +
        ((b[3] as u32) <<  0)
    }; // converts 4bytes of u8 to a u32
    reader.read_exact(&mut header_buf)?; // read the 14 bytes qoi header_buf
    file.seek(std::io::SeekFrom::Start(14))?; // reset position after use

    let width = bytes_to_u32(&header_buf[4..8]) as usize;
    let height = bytes_to_u32(&header_buf[8..12]) as usize;

    // new header on file write
    let mut header = Header::new();
    header.contents = String::from_utf8(header_buf.clone())?;
    header.width = width;
    header.height = height;
    header.format = str::from_utf8(&header_buf[0..4])?.to_owned();
    header.channels = match &header_buf[12] {
        3 => String::from("RGB"),
        4 => String::from("RGBA"),
        _ => process::exit(1), // TODO better error handling
    };
    header.colorspace = match &header_buf[13] {
        0 => String::from("sRGB with linear alpha"),
        1 => String::from("all channels linear"),
        _ => process::exit(1), // TODO
    };
    header.size = 14; // qoi always has 14 bytes headers

    Ok(header)
}
