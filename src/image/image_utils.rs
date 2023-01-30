use std::thread;
use std::thread::JoinHandle;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::Error;

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
            _ => process::exit(1)
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

// functions //

pub fn color_shift_threaded(color: &Color, data: &Vec<u8>, thread_count: usize) -> Result<Vec<u8>, Error> {
    let s_len = data.len() / thread_count; // slice length

    let mut v: Vec<Arc<Mutex<Vec<u8>>>> = Vec::new();
    for i in 0..thread_count-1 {
        v.push(Arc::new(Mutex::new(data[i*s_len..(i+1)*s_len].to_vec())));
    }
    v.push(Arc::new(Mutex::new(data[(thread_count-1)*s_len..data.len()].to_vec())));

    let mut offset: Vec<usize> = Vec::new();
    offset.push(0);
    let mut carry = 0;
    for i in 0..thread_count-1 {
        offset.push(v[i].lock().unwrap().len() % 3 + carry);
        carry = *offset.last().unwrap();
    }

    let thread = |vec: Arc<Mutex<Vec<u8>>>, p_offset: usize| {
        let c = color.clone();
        let c = [c.r, c.g, c.b];
        thread::spawn(move || {
            let mut vec = vec.lock().unwrap();
            for (i, n) in vec.iter_mut().enumerate() {
                let rgb = (i + p_offset) % 3; // gives which rgb value is queued
                if *n > 255 - c[rgb] { *n = 255; }
                else { *n += c[rgb]; }
            }
        })
    };

    let mut handle: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..thread_count {
        handle.push(thread(Arc::clone(&v[i]), offset[i]));
    }

    for h in handle.into_iter() {
        h.join().expect("thread failure");
    }

    let mut vec: Vec<u8> = Vec::new();
    for i in 0..thread_count {
        vec = [&vec[..] ,&v[i].lock().unwrap()[..]].concat();
    }

    Ok(vec)
}
