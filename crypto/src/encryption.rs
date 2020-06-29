use aes::Aes128;
use cfb8::Cfb8;
use cfb8::stream_cipher::{NewStreamCipher, StreamCipher};
use sha2::{Sha256, Digest};
use std::mem;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};

type AesCfb8 = Cfb8<Aes128>;

pub(crate) struct Crypto {
    pub(crate) encryption_mode_toggle: bool,
    pub(crate) counter: i64,
    pub(crate) aes: Option<Cfb8<Aes128>>,
    pub(crate) key: Option<Vec<u8>>,
}

pub(crate) trait CryptoT {
    fn init_state(&mut self, key: &[u8], iv: &[u8]);
    fn process(&mut self, data: &[u8]) -> &[u8];
}

impl CryptoT for Crypto {
    fn init_state(&mut self, key: &[u8], iv: &[u8]) {
        self.aes = AesCfb8::new_var(key, iv).ok();
        self.key = Option::from(Vec::from(key));
    }

    fn process(&mut self, data: &[u8]) -> &[u8] {
        let aes = self.aes.unwrap();
        let current = self.counter;
        self.counter = self.counter + 1;

        // create a Sha256 object
        let mut hasher = Sha256::new();

        if self.encryption_mode_toggle {
            let mut bs = [0u8; mem::size_of::<i64>()];
            bs.as_mut()
                .write_i64::<LittleEndian>(current)
                .expect("Unable to write");

            hasher.update(bs);
            hasher.update(data);
            hasher.update(self.key.unwrap());

            let result = &hasher.finalize()[.. 8];
            let mut input = vec![0u8; 8 + data.len()];
            input.write_all(data);
            input.write_all(&result);

            aes.encrypt(input.as_mut_slice());
            return input.as_slice()
        }

        &[]
    }
}