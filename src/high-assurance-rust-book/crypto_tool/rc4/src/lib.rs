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
        assert!(5 <= key.len() && key.len() <= 40);
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

    pub fn apply_keystream_static(key: &[u8], data: &mut [u8]) {
        let mut rc4 = Rc4::new(key);
        rc4.apply_keystream(data);
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check_dynamic() {
        #[rustfmt::skip]
        let key: [u8; 16] = [
            0x4b, 0x8e, 0x29, 0x87, 0x80, 0x95, 0x96, 0xa3,
            0xbb, 0x23, 0x82, 0x49, 0x9f, 0x1c, 0xe7, 0xc2,
        ];

        #[rustfmt::skip]
        let plaintext = [
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f,
            0x72, 0x6c, 0x64, 0x21,
        ];

        let mut rc4 = Rc4::new(&key);

        let mut msg: [u8; 12] = plaintext.clone();

        // enc
        rc4.apply_keystream(&mut msg);
        assert_ne!(msg, plaintext);
        println!(
            "Plaintext: {}\nCiphertext: {:?}\n",
            String::from_utf8(plaintext.to_vec()).unwrap(),
            msg
        );

        // dec
        rc4 = Rc4::new(&key);
        rc4.apply_keystream(&mut msg);
        assert_eq!(msg, plaintext);
        println!("Decrypted: {}", String::from_utf8(msg.to_vec()).unwrap());
    }

    #[test]
    fn sanity_check_static_encr() {
        #[rustfmt::skip]
        let key: [u8; 16] = [
            0x4b, 0x8e, 0x29, 0x87, 0x80, 0x95, 0x96, 0xa3,
            0xbb, 0x23, 0x82, 0x49, 0x9f, 0x1c, 0xe7, 0xc2,
        ];

        #[rustfmt::skip]
        let plaintext = [
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f,
            0x72, 0x6c, 0x64, 0x21,
        ];

        let mut msg: [u8; 12] = plaintext.clone();
        println!("Plaintext: `{}`", String::from_utf8(msg.to_vec()).unwrap());

        // enc:
        Rc4::apply_keystream_static(&key, &mut msg);
        assert_ne!(msg, plaintext);
        println!("Ciphertext: `{:?}`", msg);

        // dec
        Rc4::apply_keystream_static(&key, &mut msg);
        assert_eq!(msg, plaintext);
        println!("Decrypted: `{}`", String::from_utf8(msg.to_vec()).unwrap());
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
