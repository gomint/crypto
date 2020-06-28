use std::io::{self, Read, Write};
use libflate::deflate::{Encoder, Decoder};

pub async fn decompress(encoded_data: Vec<u8>) -> io::Result<Vec<u8>> {
    // Decoding
    let mut decoder = Decoder::new(&encoded_data[..]);
    let mut decoded_data = Vec::new();
    let result = decoder.read_to_end(&mut decoded_data);

    // Check for error
    if result.is_err() {
        Err(result.err().unwrap())
    } else {
        Ok(decoded_data)
    }
}

pub async fn compress(decoded_data: Vec<u8>) -> io::Result<Vec<u8>> {
    // Decoding
    let mut encoder = Encoder::new(Vec::new());
    encoder.write_all(&decoded_data[..]).unwrap();

    encoder.finish().into_result()
}