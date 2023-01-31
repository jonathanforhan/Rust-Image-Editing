mod image_header;
use image_header::ImageHeader;
mod image_log;
use image_log::image_log;
mod image_utils;
pub use image_utils::Color;
mod color_shift;
use color_shift::color_shift_threaded;

use std::{
    fs::File,
    io::{Read, Write},
    error::Error,
	sync::{Arc, RwLock},
};

type DataRef = Arc<RwLock<Vec<u8>>>;

#[derive(Debug, Clone)]
pub struct Image {
    header: ImageHeader,
    data: DataRef,
}

impl Image {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path).expect("Unable to open file");
        let image_header = ImageHeader::new(&file).expect("Header generation failed");
        let data: DataRef = Arc::new(RwLock::new(Vec::new()));

        file.read_to_end(&mut data.write().unwrap()).expect("Unable to read file");

        //image_log(&image_header, data.read().unwrap().len());
        Ok(Image { header: image_header.clone(), data })
    }

    pub fn color_shift(&mut self, color: Color) -> std::io::Result<()> {
        let min = color.clone().into_iter().min().unwrap();
        let color = color.into_iter().map(|x| x - min).collect::<Color>();

        color_shift_threaded(&color, Arc::clone(&self.data), 4).unwrap();

        Ok(())
    }

    #[allow(dead_code)]
    pub fn compress(&mut self) -> std::io::Result<()> {
        // TODO impliment compression by blending the pixels some thing
        // for n in self.iter() {
        //     add_to_buffer(n, n+1, n+width, n+width+1)
        //     blend(buffer)
        //     i += 2;
        // }
        // maybe double for loop with a 2width and 2pixel gap for 4x4 blending

        Ok(())
    }

    pub fn write_file(&self, file_name: &str) -> std::io::Result<()> {
        let mut file = File::create(file_name.to_string() + ".ppm")?;
        let header = self.header.fmt()?;

        file.write(header.as_bytes())?;
        file.write(&self.data.read().unwrap())?;

        Ok(())
    }
}
