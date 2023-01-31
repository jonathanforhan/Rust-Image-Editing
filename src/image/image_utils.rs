use std::process;

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

