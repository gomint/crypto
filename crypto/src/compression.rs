use std::io::{Read, Write};
use libflate::deflate::{Encoder, Decoder};

pub fn decompress(data: &[u8]) -> Vec<u8> {
    // Decoding
    let mut decoder = Decoder::new(&data[..]);
    let mut decoded_data = Vec::new();
    let result = decoder.read_to_end(&mut decoded_data);

    // Check for error
    if result.is_err() {
        return Vec::with_capacity(0)
    } else {
        decoded_data
    }
}

pub fn compress(data: &[u8]) -> Vec<u8> {
    // Compression
    let mut encoder = Encoder::new(Vec::new());
    encoder.write_all(&data[..]).unwrap();

    // Return slice
    encoder.finish().into_result().unwrap()
}