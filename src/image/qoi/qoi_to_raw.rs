use std::error::Error;

use crate::image::Header;

type Pixel = [u8; 4]; // 4 bytes

const QOI_OP_RGB: u8   = 0b1111_1110; // 8-bit tags
const QOI_OP_RGBA: u8  = 0b1111_1111;

const QOI_OP_INDEX: u8 = 0b00_111111; // 2-bit tags
const QOI_OP_DIFF: u8  = 0b01_111111; // trailing ones are for easier
const QOI_OP_LUMA: u8  = 0b10_111111; // bitwise operations
const QOI_OP_RUN: u8   = 0b11_111111;

fn qoi_hash(px: &Pixel) -> usize {
    let r = px[0] as usize * 3;
    let g = px[1] as usize * 5;
    let b = px[2] as usize * 7;
    let a = px[3] as usize * 11;
    r + g + b + a
}

pub fn qoi_to_raw(header: &mut Header, mut data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut new_data: Vec<u8> = Vec::new();
    let mut index: [Pixel; 64] = [[0; 4]; 64];

    let step = match &header.channels[..] {
        "RGB" => 3,
        "RGBA" => 4,
        _ => { return Err(Box::<dyn Error>::from("Unsupported color channels")) },
    };
    let mut px: Pixel = [0, 0, 0, 0xff];

    let mut i = 0;
    loop {
        if i == data.len()-8 { break; } // qoi has 8 bytes of EOF padding hence subtraction
        if data[i] == QOI_OP_RGB {
            px[0] = data[i+1];
            px[1] = data[i+2];
            px[2] = data[i+3];
            i+=4;
        } else if data[i] == QOI_OP_RGBA {
            px[0] = data[i+1];
            px[1] = data[i+2];
            px[2] = data[i+3];
            px[3] = data[i+4];
            i+=5;
        } else {
            let tag = data[i] | 0b00_111111u8;
            match tag {
                QOI_OP_INDEX => {
                    px = index[data[i] as usize];
                    i+=1
                },
                QOI_OP_DIFF => {
                    px[0] = px[0].wrapping_add(((data[i] >> 4) & 0x03).wrapping_sub(2));
                    px[1] = px[1].wrapping_add(((data[i] >> 2) & 0x03).wrapping_sub(2));
                    px[2] = px[2].wrapping_add(( data[i]       & 0x03).wrapping_sub(2));
                    i+=1
                }
                QOI_OP_LUMA => {
                    let green = (data[i] & 0x3f).wrapping_sub(32);
                    px[0] = px[0].wrapping_add(green.wrapping_sub(8).wrapping_add((data[i+1] >> 4) & 0x0f));
                    px[1] = px[1].wrapping_add(green);
                    px[2] = px[2].wrapping_add(green.wrapping_sub(8).wrapping_add(data[i+1] & 0x0f));
                    i+=2;
                },
                QOI_OP_RUN => {
                    new_data.extend_from_slice(&px[..step]);
                    for _ in 0..(data[i] ^& 0xc0) {
                        new_data.extend_from_slice(&px[..step]);
                    }
                    i+=1;
                    continue;
                },
                _ => panic!("Byte tag error")
            }
        }
        index[qoi_hash(&px) % 64] = px;
        new_data.extend_from_slice(&px[..step]);
    }
    data = new_data;
   
    Ok(data)
}
