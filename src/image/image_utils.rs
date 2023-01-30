use std::thread;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;

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

pub fn color_shift_threaded(color: &Color, data: &Vec<u8>) -> std::io::Result<Vec<u8>> {
    let s_len = data.len() / 8; // slice length

    let v1 = Arc::new(Mutex::new(data[0*s_len..1*s_len].to_vec()));
    let v2 = Arc::new(Mutex::new(data[1*s_len..2*s_len].to_vec()));
    let v3 = Arc::new(Mutex::new(data[2*s_len..3*s_len].to_vec()));
    let v4 = Arc::new(Mutex::new(data[3*s_len..4*s_len].to_vec()));
    let v5 = Arc::new(Mutex::new(data[4*s_len..5*s_len].to_vec()));
    let v6 = Arc::new(Mutex::new(data[5*s_len..6*s_len].to_vec()));
    let v7 = Arc::new(Mutex::new(data[6*s_len..7*s_len].to_vec()));
    let v8 = Arc::new(Mutex::new(data[7*s_len..data.len()].to_vec()));

    let o1 = 0;
    let o2 = v1.lock().unwrap().len() % 3;
    let o3 = v2.lock().unwrap().len() % 3 + o2;
    let o4 = v3.lock().unwrap().len() % 3 + o3;
    let o5 = v4.lock().unwrap().len() % 3 + o4;
    let o6 = v5.lock().unwrap().len() % 3 + o5;
    let o7 = v6.lock().unwrap().len() % 3 + o6;
    let o8 = v7.lock().unwrap().len() % 3 + o7;

    let thread = |vec: Arc<Mutex<Vec<u8>>>, offset: usize| {
        let c = color.clone();
        let c = [c.r, c.g, c.b];
        thread::spawn(move || {
            let mut vec = vec.lock().unwrap();
            for (i, n) in vec.iter_mut().enumerate() {
                let rgb = (i + offset) % 3; // gives which rgb value is queued
                if *n > 255 - c[rgb] { *n = 255; }
                else { *n += c[rgb]; }
            }
        })
    };

    let h1 = thread(Arc::clone(&v1), o1);
    let h2 = thread(Arc::clone(&v2), o2);
    let h3 = thread(Arc::clone(&v3), o3);
    let h4 = thread(Arc::clone(&v4), o4);
    let h5 = thread(Arc::clone(&v5), o5);
    let h6 = thread(Arc::clone(&v6), o6);
    let h7 = thread(Arc::clone(&v7), o7);
    let h8 = thread(Arc::clone(&v8), o8);

    h1.join().expect("thread failure");
    h2.join().expect("thread failure");
    h3.join().expect("thread failure");
    h4.join().expect("thread failure");
    h5.join().expect("thread failure");
    h6.join().expect("thread failure");
    h7.join().expect("thread failure");
    h8.join().expect("thread failure");

    let vec = [&v1.lock().unwrap()[..],
               &v2.lock().unwrap()[..],
               &v3.lock().unwrap()[..],
               &v4.lock().unwrap()[..],
               &v5.lock().unwrap()[..],
               &v6.lock().unwrap()[..],
               &v7.lock().unwrap()[..],
               &v8.lock().unwrap()[..]].concat();

    Ok(vec)
}
