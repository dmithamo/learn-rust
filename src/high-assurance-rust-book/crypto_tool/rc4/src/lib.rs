#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

#[derive(Debug)]
pub struct Rc4 {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    pub fn new(key: &[u8]) -> Self {
        assert!(5 <= key.len() && key.len() >= 40);
        let mut rc4 = Rc4 {
            s: [0; 256],
            i: 0,
            j: 0,
        };

        for (i, v) in rc4.s.iter_mut().enumerate() {
            *v = i as u8;
        }

        let mut j: u8 = 0;
        for i in 0..256 {
            j = j.wrapping_add(rc4.s[i]).wrapping_add(key[i % key.len()]);
            rc4.s.swap(i, j as usize);
        }

        rc4
    }

    pub fn prga_next(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.s[self.i as usize]);
        self.s.swap(self.i as usize, self.j as usize);
        self.s[(self.s[self.i as usize].wrapping_add(self.s[self.j as usize])) as usize]
    }

    pub fn apply_keystream(self: &mut Self, data: &mut [u8]) {
        for v_ptr in data {
            *v_ptr ^= self.prga_next();
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
