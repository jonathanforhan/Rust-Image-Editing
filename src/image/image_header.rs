use std::{
    fs::File,
    io::{BufReader, BufRead, Seek},
    error::Error
};

#[derive(Debug, Clone)]
pub struct ImageHeader {
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub encoding: String,
    pub size: usize,
}

impl ImageHeader {
    pub fn new(mut file: &File) -> Result<Self, Box<dyn Error>> {
        let reader = BufReader::new(file);
        let mut image_header = ImageHeader { format:   String::new(),
                                             width:    0,
                                             height:   0,
                                             encoding: String::new(),
                                             size:     0 };
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
                1 => image_header.format = s.to_owned(),
                2 => { 
                    let mut split = s.split(" ");
                    image_header.width = split.next().expect("Width").parse()?;
                    image_header.height = split.next().expect("Height").parse()?;
                },
                3 => {
                    image_header.encoding = s.parse()?;
                    image_header.size = header_size;
                    break;
                },
                _ => break, // unreachable but gives a warm and fuzzy feeling
            };
        };
        file.seek(std::io::SeekFrom::Start(header_size as u64))?; // reset position after use
        Ok(image_header)
    }

    pub fn fmt(&self) -> std::io::Result<String> {
        let header = self.format.clone() + "\n" +
                     &self.width.to_string() + " " +
                     &self.height.to_string() + "\n" +
                     &self.encoding.to_string() + "\n";
        Ok(header)

    }
}

