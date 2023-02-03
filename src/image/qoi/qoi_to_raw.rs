use std::{error::Error, process};

use crate::image::Header;

type Pixel = [u8; 4]; // 4 bytes

const QOI_OP_RGB: u8   = 0b1111_1110u8; // 8-bit tags
const QOI_OP_RGBA: u8  = 0b1111_1111u8;

const QOI_OP_INDEX: u8 = 0b00_111111u8; // 2-bit tags
const QOI_OP_DIFF: u8  = 0b01_111111u8; // trailing ones are for easier
const QOI_OP_LUMA: u8  = 0b10_111111u8; // bitwise operations
const QOI_OP_RUN: u8   = 0b11_111111u8;

#[allow(unused)]
pub fn qoi_to_raw(header: &mut Header, mut data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let qoi_hash = |pix: &Pixel| -> usize {
        ((pix[0]as u16 *3 + pix[1]as u16 *5 + pix[2]as u16 *7 + pix[3]as u16 *11) % 64) as usize
    };
    let mut index: [Pixel; 64] = [[0; 4]; 64]; // initialize Pixel[64] array to 0s
    let mut new_data: Vec<u8> = Vec::new();

    let step = match &header.channels[..] {
        "RGB" => 3,
        "RGBA" => 4,
        _ => process::exit(1), // TODO 
    };
    let data_len = header.width * header.height * step;
    new_data.reserve_exact(data_len);

    let mut pixel = [0, 0, 0, 0xFFu8]; // current working pixel

    println!("Start loop");
    for mut i in (0..data.len()).step_by(1) {
        // compare 8-bit tag first as per spec
        if data[i] == QOI_OP_RGB {
            pixel[0] = data[i+1];
            pixel[1] = data[i+2];
            pixel[2] = data[i+3];
            i += 2;
        } else if data[i] == QOI_OP_RGBA {
            pixel[0] = data[i+1];
            pixel[1] = data[i+2];
            pixel[2] = data[i+3];
            pixel[3] = data[i+4];
            i += 3;
        } else {
            // if no 8-bit tag compare 2-bit
            let bit_tag = data[i] | 0b00_111111u8;

            match bit_tag {
                QOI_OP_INDEX => {
                    let index_pos = data[i] as usize;
                    pixel = index[index_pos];
                },
                QOI_OP_DIFF => {
                    pixel[0].wrapping_add((data[i] >> 4 & 0x03).wrapping_sub(2));
                    pixel[1].wrapping_add((data[i] >> 2 & 0x03).wrapping_sub(2));
                    pixel[2].wrapping_add((data[i] & 0x03).wrapping_sub(2));
                },
                QOI_OP_LUMA => {
                    let green = (data[i] & 0x3F).wrapping_sub(32);
                    pixel[0].wrapping_add((green.wrapping_sub(8)).wrapping_add(data[i+1] >> 4 & 0x0F));
                    pixel[1].wrapping_add(green);
                    pixel[2].wrapping_add((green.wrapping_sub(8)).wrapping_add(data[i+1] & 0x0F));
                    i += 1;
                },
                QOI_OP_RUN => {
                    let run = data[i] & 0x3F;

                    for _ in 0..run+1 {
                        new_data.extend_from_slice(&pixel[0..step]);
                    }
                },
                _ => panic!("Read error"),
            }
        }
        new_data.extend_from_slice(&pixel[0..step]); // auto correcting for RGB vs RGBA
        let index_pos = qoi_hash(&pixel);
        index[index_pos] = pixel;
    }
    new_data.extend_from_slice(&[0,0,0,0,0,0,0,1]);
    println!("End loop");
    data = new_data;

    Ok(data)
}
