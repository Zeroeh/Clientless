pub fn new_key_pair(out: &[u8], inc: &[u8]) -> CipherPair {
    CipherPair::new(out, inc)
}

pub const OUTGOING_KEY: &[u8; 13] = b"\x6a\x39\x57\x0c\xc9\xde\x4e\xc7\x1d\x64\x82\x18\x94";
pub const INCOMING_KEY: &[u8; 13] = b"\xc7\x93\x32\xb1\x97\xf9\x2b\xa8\x5e\xd2\x81\xa0\x23";

#[derive(Clone)]
pub struct CipherPair {
    pub outgoing: RC4Cipher,
    pub incoming: RC4Cipher,
}

impl CipherPair {
    pub fn new(outkey: &[u8], inkey: &[u8]) -> CipherPair {
        CipherPair {
            outgoing: RC4Cipher::new(outkey),
            incoming: RC4Cipher::new(inkey),
        }
    }
    pub fn reset(&mut self) {
        self.incoming.reset();
        self.outgoing.reset();
    }
    pub fn new_const() -> CipherPair {
        CipherPair {
            outgoing: RC4Cipher::new(OUTGOING_KEY),
            incoming: RC4Cipher::new(INCOMING_KEY),
        }
    }
}

#[derive(Clone)]
pub struct RC4Cipher {
    i: u8,
    j: u8,
    s: [u8; 256],
}

impl RC4Cipher {
    /// Creates a new RC4 cipher from the input key
    fn new(key: &[u8]) -> RC4Cipher {
        let k = key.len();
        if k < 1 || k > 256 {
            panic!("Key size invalid");
        }
        let mut cipher = RC4Cipher {
            //zero initialize everything
            i: 0,
            j: 0,
            s: [0; 256],
        };
        for i in 0..256 {
            cipher.s[i] = i as u8;
        }
        let mut j: u8 = 0;
        for i in 0..256 {
            // j += (cipher.s[i] as u8) + key[i % k];
            j = (j.wrapping_add(cipher.s[i] as u8)).wrapping_add(key[i % k]);
            cipher.s.swap(i, j as usize);
        }
        cipher
    }
    /// Not sure if this is required, but adding it for implementations sake
    pub fn reset(&mut self) {
        for i in 0..self.s.len() {
            self.s[i] = 0;
        }
        self.i = 0;
        self.j = 0;
    }
    /// Generic function to xor the buffer with the cipher. Generally, use a higher level wrapper over this.
    pub fn xor_key_stream(&mut self, dst: &mut [u8], src: &mut [u8]) {
        let mut i = self.i;
        let mut j = self.j;
        let mut idx = 0;
        for v in src.iter() {
            i = i.wrapping_add(1);
            j = j.wrapping_add(self.s[i as usize] as u8);
            self.s.swap(i as usize, j as usize);
            let t = self.s[(self.s[i as usize].wrapping_add(self.s[j as usize])) as usize] as u8;
            dst[idx] = v ^ t;
            idx += 1;
        }
        // assert_eq!(idx, dst.len()); //check to make sure we guud
        self.i = i;
        self.j = j;
    }
}
