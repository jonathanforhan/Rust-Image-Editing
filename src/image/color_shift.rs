use std::{
    io::ErrorKind,
    thread,
	thread::JoinHandle,
	sync::{Arc, RwLock},
};
use super::Color;

type DataRef = Arc<RwLock<Vec<u8>>>;

pub fn color_shift_threaded(color: &Color, data: DataRef, thread_count: usize) -> Result<(), ErrorKind> {
    if thread_count < 1 { return Err(ErrorKind::InvalidInput); }
    let s_len = data.read().unwrap().len() / thread_count; // slice length

    // calculate pixel offset for each row that thread will be working on
    let mut offset: Vec<usize> = Vec::from([0]); // index [0] is 0
    let mut carry = 0; // carry the prev offset
    for i in 0..thread_count-1 {
        offset.push(data.read().unwrap()[(i * s_len)..((i+1) * s_len)].len() % 3 + carry);
        carry = *offset.last().unwrap();
    }

    // multi threaded color shift
    let thread = |p_vec: DataRef, p_slice: (usize, usize), p_offset: usize| {
        let c = color.clone();
        let c = [c.r, c.g, c.b]; // makes it indexible with implimenting it on struct
        thread::spawn(move || {
            for (i, n) in p_vec.write().unwrap()[p_slice.0..p_slice.1].iter_mut().enumerate() {
                let rgb = (i + p_offset) % 3; // gives which rgb value is queued
                if *n > 255 - c[rgb] { *n = 255; } // bounds check
                else { *n += c[rgb]; }
            }
        })
    };
    
    // call the threads
    let mut handle: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..thread_count-1 {
        handle.push(thread(Arc::clone(&data), (i * s_len, (i+1) * s_len), offset[i]));
    }
    handle.push(thread(Arc::clone(&data), ((thread_count-1) * s_len, data.read().unwrap().len()), offset[offset.len()-1]));

    // join them back
    let _ = handle.into_iter().map(|h| h.join().expect("Thread failure"));

    Ok(())
}
