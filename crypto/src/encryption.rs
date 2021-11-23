use sha2::Digest;
use std::mem;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};
use std::borrow::BorrowMut;
use aes::cipher::errors::InvalidLength;
use aes::cipher::NewCipher;
use ctr::cipher::StreamCipher;

use crate::context::Context;

type AesCtr = ctr::Ctr128BE<aes::Aes256>;

pub(crate) trait CryptoT {
    fn init_state(&mut self, key: &[u8], iv: &[u8]);
    fn process(&mut self, data: &mut [u8]) -> Box<Vec<u8>>;
}

impl CryptoT for Context {
    fn init_state(&mut self, key: &[u8], iv: &[u8]) {
        let a: Result<AesCtr, InvalidLength> = AesCtr::new_from_slices(key, iv);
        if a.is_err() {
            println!("Could not init aes: invalid key length {}", key.len());
        }

        self.aes = a.ok();
        self.key = Option::from(Vec::from(key));
    }

    fn process(&mut self, data: &mut [u8]) -> Box<Vec<u8>> {
        if self.aes.is_none() {
            return Box::from(data.to_vec())
        }

        let aes = self.aes.as_mut().unwrap();
        let current = self.counter;
        self.counter = self.counter + 1;

        // Write the counter as LE
        let mut bs = [0u8; mem::size_of::<i64>()];
        bs.as_mut()
            .write_i64::<LittleEndian>(current)
            .expect("Unable to write");

        let hasher = self.digest.borrow_mut();

        if self.encryption_mode_toggle {
            hasher.update(bs);
            hasher.update(data.as_ref());
            hasher.update(self.key.as_ref().unwrap());

            let result = &hasher.finalize_reset()[..8];

            let mut input: Box<Vec<u8>>  = Box::from(Vec::new());
            input.write_all(data.as_ref()).unwrap();
            input.write_all(&result).unwrap();

            aes.apply_keystream(input.as_mut_slice());
            return input;
        }

        // decrypt first
        aes.apply_keystream(data);

        let offset = data.len() - 8;

        hasher.update(bs);
        hasher.update(&data[..offset]);
        hasher.update(self.key.as_ref().unwrap());

        let expected = &data[offset..];
        let result = &hasher.finalize_reset()[..8];

        if expected != result {
            println!("Not matching hash: {:x?} / {:x?}", expected, result);
            return Box::new(Vec::new())
        }

        return Box::from(data[..offset].to_vec())
    }

}