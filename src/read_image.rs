use std::{
    fs::File,
    io::{BufReader, BufRead, Read},
    error::Error
};

#[derive(Debug, Clone)]
struct ImageHeader {
    width: u32,
    height: u32,
    encoding: String,
}

impl ImageHeader {
    fn new(height: u32, width: u32, encoding: String) -> Self {
        ImageHeader { height, width, encoding }
    }

    fn build(file: &File) -> Result<Self, Box<dyn Error>> {
        let buf = BufReader::new(file);
        let mut image_header = ImageHeader::new(0, 0, String::from("null"));
        let mut file_iter = buf.lines().into_iter();
        file_iter.next();

        let s = file_iter.next().unwrap()?;
        let v: Vec<&str> = s.split(" ").collect();

        image_header.width = v[0].parse()?;
        image_header.height = v[1].parse()?;

        let s = file_iter.next().unwrap()?;
        image_header.encoding = s.parse()?;

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

        Ok(Image::new(image_header.clone(), data))
    }

    pub fn print(&self) {
        println!("Width: {}px", self.header.width);
        println!("Height: {}px", self.header.height);
        println!("Encoding: {}", self.header.encoding);
        println!("bytes: {}", self.data.len());
    }
}
