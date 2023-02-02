use std::{
    fs::File,
    io::{BufReader, BufRead, Seek},
    error::Error, process
};

use crate::image::Header;

pub fn ppm_header(mut file: &File) -> Result<Header, Box<dyn Error>> {
    let reader = BufReader::new(file);
    let mut header = Header::new();
    let mut file_iter = reader.lines().into_iter();

    let mut header_size = 0;
    let mut i = 0;
    loop {
        let s = file_iter.next().unwrap()?;
        header_size += s.len()+1; // +1 adding newline char
        let s = s.trim();
        if s.starts_with('#') || s.starts_with('\n') || s.is_empty() { continue; }
        i += 1;
        match i {
            1 => header.format = s.to_owned(),
            2 => { 
                let mut split = s.split(" ");
                header.width = split.next().expect("Width").parse()?;
                header.height = split.next().expect("Height").parse()?;
            },
            3 => {
                header.encoding = s.parse()?;
                header.size = header_size;
                break;
            },
            _ => process::exit(1)
        };
    };
    file.seek(std::io::SeekFrom::Start(header_size as u64))?; // reset position after use
    
    // new header on file write
    header.contents = header.format.clone() + "\n" +
                      &header.width.to_string() + " " +
                      &header.height.to_string() + "\n" +
                      &header.encoding.to_string() + "\n";
    Ok(header)
}
