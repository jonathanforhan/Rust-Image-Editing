mod ppm;
mod qoi;
mod utils;
pub use utils::{ImgFormat, Color, Header};
use crate::image::qoi::qoi_to_raw;

use std::{
    fs::File,
    io::{Read, Write},
    error::Error,
    fmt::Debug,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Image {
    header: Header,
    data: Vec<u8>,
    format: ImgFormat,
}

impl Image {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let format = match &path[path.len()-4..] {
            ".ppm" => ImgFormat::Ppm,
            ".qoi" => ImgFormat::Qoi,
            ".png" => ImgFormat::Png,
            ".jpg" | ".jpeg" => ImgFormat::Jpg,
            _ => { return Err(Box::<dyn Error>::from("Unsupported file extension")); },
        };
        let mut file = File::open(path)?;
        let mut header = Header::from(&format, &file)?;
        let mut data: Vec<u8> = Vec::new();

        file.read_to_end(&mut data)?;

        match format {
            ImgFormat::Ppm => (),
            ImgFormat::Qoi => { data = qoi_to_raw(&mut header, data)?; () },
            ImgFormat::Png => { return Err(Box::<dyn Error>::from("Unsupported format")); }, // TODO
            ImgFormat::Jpg => { return Err(Box::<dyn Error>::from("Unsupported format")); }, // TODO
        }

        Ok(Image { header, data, format })
    }

    pub fn color_shift(&mut self, color: (u8, u8, u8)) -> Result<&mut Self, Box<dyn Error>> {
        let color = Color::new(color.0, color.1, color.2);
        let min = color.clone().into_iter().min().unwrap();
        let color = color.into_iter().map(|x| x - min).collect::<Color>();

        let step = match &self.header.channels[..] {
            "RGB" => 3,
            "RGBA" => 4,
            _ => { return Err(Box::<dyn Error>::from("Unsupported color channels")); },
        };

        let c = [color.r, color.g, color.b]; // makes it indexible without implimenting it on Color struct
        for (i, n) in self.data.iter_mut().enumerate() {
            let rgb = i % step; // gives which rgb value is queued
            if rgb == 3 { continue; } // alpha value
            if *n > 255 - c[rgb] { *n = 255; } // bounds check
            else { *n += c[rgb]; }
        }

        Ok(self)
    }

    pub fn convert(&mut self, format: ImgFormat) -> Result<&mut Self, Box<dyn Error>> {
        if self.format == format { return Err(Box::<dyn Error>::from("Invalid conversion to self")); }
        self.format = format;
        
        Ok(self)
    }

    pub fn write_file(&mut self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let extension = match &self.format {
            ImgFormat::Ppm => {
                self.data = ppm::raw_to_ppm(&mut self.header, &mut self.data).unwrap();
                ".ppm"
            },
            ImgFormat::Qoi => ".qoi", // raw_to_qoi();
            _ => return Err(Box::<dyn Error>::from("Invalid format")),
        };
        let mut file = File::create(file_name.to_string() + extension)?;

        file.write(self.header.contents.as_bytes())?;
        file.write(&self.data)?;

        Ok(())
    }
}
