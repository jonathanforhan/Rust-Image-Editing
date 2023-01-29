use std::{
    fs::File,
    io::{BufReader, BufRead, Read, Write},
    error::Error, process
};

#[derive(Debug, Clone)]
struct ImageHeader {
    format: String,
    width: u32,
    height: u32,
    encoding: String,
    size: usize,
}

impl ImageHeader {
    fn new(format: String,height: u32, width: u32, encoding: String, size: usize) -> Self {
        ImageHeader { format, height, width, encoding, size }
    }

    fn build(file: &File) -> Result<Self, Box<dyn Error>> {
        let reader = BufReader::new(file);
        let mut image_header = ImageHeader::new(String::new(), 0, 0, String::new(), 0);
        let mut file_iter = reader.lines().into_iter();

        let mut header_size = 0;
        let mut i = 0;
        loop {
            let s = file_iter.next().unwrap()?;
            header_size += s.len()+1;
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
        Ok(image_header)
    }
}

#[derive(Debug)]
pub struct Image {
    header: ImageHeader,
    data: Vec<u8>,
}

impl Image {
    fn new(header: ImageHeader, data: Vec<u8>) -> Self {
        Image { header, data }
    }

    pub fn build(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path).expect("Unable to open file");
        let image_header = ImageHeader::build(&file).expect("Header generation failed");
        let mut file = File::open(path).expect("Unable to open file");
        let mut data: Vec<u8> = Vec::new();

        file.read_to_end(&mut data).expect("Unable to read file");
        data.drain(0..image_header.size); // trim off header

        println!("Format: {}", image_header.format);
        println!("Width: {}px", image_header.width);
        println!("Height: {}px", image_header.height);
        println!("Encoding: {}", image_header.encoding);
        println!("File size: {} bytes", data.len() + image_header.size);

        Ok(Image::new(image_header.clone(), data))
    }

    pub fn color_shift(&mut self, color: &str, fname: &str, degree: u8) -> std::io::Result<()> {
        let shift = match color {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => process::exit(1),
        };
        // shift nth rgb value
        for (i, n) in self.data.iter_mut().enumerate() {
            if (i+3 - shift) % 3 == 0 {
                if *n > 255 - degree { *n = 255; }
                else { *n += degree; }
            }
        }

        let mut file = File::create(fname.to_string() + ".ppm")?;
    
        let header = self.header.format.clone() + "\n" +
                     &self.header.width.to_string() + " " +
                     &self.header.height.to_string() + "\n" +
                     &self.header.encoding.to_string() + "\n";

        file.write(header.as_bytes())?;
        file.write(&self.data)?;

        Ok(())
    }
}
