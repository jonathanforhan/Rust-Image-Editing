use std::{
    process,
    error::Error,
    fs::File,
};
use super::ppm::ppm_header;
use super::qoi::qoi_header;

#[derive(Debug, PartialEq, Eq)]
pub enum ImgFormat {
    Jpg,
    Png,
    Ppm,
    Qoi,
}

#[allow(dead_code)]
pub enum VidFormat {
    Mp4,
    Wav,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub contents: String,   // all formats
    pub width: usize,         // all formats
    pub height: usize,        // all formats
    pub format: String,     // all formats
    pub encoding: String,   // ppm
    pub channels: String,   //     qoi
    pub colorspace: String, //     qoi
    pub size: usize,        // all formats
}

impl Header {
    pub fn from(img_format: &ImgFormat, file: &File) -> Result<Self, Box<dyn Error>> {
        match img_format {
            ImgFormat::Ppm => Ok(ppm_header(file)?),
            ImgFormat::Qoi => Ok(qoi_header(file)?),
            _ => { return Err(Box::<dyn Error>::from("Invalid file format")); }
        }
    }

    pub fn new() -> Self {
        Header {
            contents: String::new(),
            width: 0,
            height: 0,
            format: String::new(),
            encoding: String::new(),
            channels: String::new(),
            colorspace: String::new(),
            size: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    iter: usize,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color{ r, g, b, iter: 0 }
    }

    fn add(&mut self, elem: u8) {
        match self.iter {
            0 => self.r = elem,
            1 => self.g = elem,
            2 => self.b = elem,
            _ => process::exit(1) // should never happen
        }
        self.iter += 1;
    }
}

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 3>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.r, self.g, self.b])
    }
}

impl FromIterator<u8> for Color {
    fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Self {
        let mut c = Color::new(0, 0, 0);

        for i in iter { c.add(i); }
        c
    }
}


