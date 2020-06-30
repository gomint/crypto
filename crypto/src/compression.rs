use std::io::{Read, Write};
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;

pub fn decompress(data: &[u8]) -> Box<Vec<u8>> {
    // Decoding
    let mut decoder = DeflateDecoder::new(&data[..]);
    let mut decoded_data = Box::new(Vec::new());
    let result = decoder.read_to_end(&mut decoded_data);

    // Check for error
    if result.is_err() {
        return Box::new(Vec::with_capacity(0))
    } else {
        decoded_data
    }
}

pub fn compress(data: &[u8]) -> Vec<u8> {
    // Compression
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data[..]).unwrap();

    // Return slice
    encoder.finish().unwrap()
}