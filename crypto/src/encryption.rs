use aes::Aes256;
use cfb8::Cfb8;
use cfb8::stream_cipher::{NewStreamCipher, StreamCipher, InvalidKeyNonceLength};
use sha2::{Sha256, Digest};
use std::mem;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};

type AesCfb8 = Cfb8<Aes256>;

#[repr(C)]
pub(crate) struct Crypto {
    pub(crate) encryption_mode_toggle: bool,
    pub(crate) counter: i64,
    pub(crate) aes: Option<Cfb8<Aes256>>,
    pub(crate) key: Option<Vec<u8>>,
}

pub(crate) trait CryptoT {
    fn init_state(&mut self, key: &[u8], iv: &[u8]);
    fn process(&mut self, data: &mut [u8]) -> Box<Vec<u8>>;
}

impl AsMut<Crypto> for Crypto {
    fn as_mut(&mut self) -> &mut Crypto {
        self
    }
}

impl CryptoT for Crypto {
    fn init_state(&mut self, key: &[u8], iv: &[u8]) {
        let a: Result<AesCfb8, InvalidKeyNonceLength> = AesCfb8::new_var(key, iv);
        if a.is_err() {
            println!("Could not init aes: invalid key length {}", key.len());
        }

        self.aes = a.ok();
        self.key = Option::from(Vec::from(key));

        println!("Crypto state on init: {} :: {} :: {} :: {}", self.encryption_mode_toggle, self.counter, self.key.is_some(), self.aes.is_some());
    }

    fn process(&mut self, data: &mut [u8]) -> Box<Vec<u8>> {
        println!("Crypto state on init: {} :: {} :: {} :: {}", self.encryption_mode_toggle, self.counter, self.key.is_some(), self.aes.is_some());
        println!("Got input data: {:x?}", data);

        if self.aes.is_none() {
            println!("Crypto not enabled, we passthrough");
            return Box::from(data.to_vec())
        }

        let aes = self.aes.as_mut().unwrap();
        let current = self.counter;
        self.counter = self.counter + 1;

        // create a Sha256 object
        let mut hasher = Sha256::new();

        // Write the counter as LE
        let mut bs = [0u8; mem::size_of::<i64>()];
        bs.as_mut()
            .write_i64::<LittleEndian>(current)
            .expect("Unable to write");

        if self.encryption_mode_toggle {
            hasher.update(bs);
            hasher.update(data.as_ref());
            hasher.update(self.key.as_ref().unwrap());

            let result = &hasher.finalize()[..8];

            println!("Got input data (after hash): {:x?}, {}", data, data.len());

            let mut input: Box<Vec<u8>>  = Box::from(Vec::new());
            input.write_all(data.as_ref()).unwrap();
            input.write_all(&result).unwrap();

            println!("Output data: {:x?}", input);
            println!("Hash for data: {:x?}", result);

            aes.encrypt(input.as_mut_slice());
            return input;
        }

        // decrypt first
        aes.decrypt(data);

        let offset = data.len() - 8;

        hasher.update(bs);
        hasher.update(&data[..offset]);
        hasher.update(self.key.as_ref().unwrap());

        let expected = &data[offset..];
        let result = &hasher.finalize()[..8];

        if expected != result {
            println!("Incoming data: {:x?}", data);
            println!("Not matching hash: {:x?} / {:x?}", expected, result);
            return Box::new(Vec::new())
        }

        return Box::from(data[..offset].to_vec())
    }
}