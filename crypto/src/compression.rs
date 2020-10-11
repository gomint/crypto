use libdeflater::{Compressor, CompressionLvl, Decompressor};

pub fn decompress(data: &[u8]) -> Box<Vec<u8>> {
    // Decoding
    let mut decoder = Decompressor::new();
    let mut decoded_data = Box::new(Vec::new());
    decoded_data.resize(2*1024*1024, 0);
    let result = decoder.deflate_decompress(data, decoded_data.as_mut_slice());

    // Check for error
    if result.is_err() {
        return Box::new(Vec::with_capacity(0))
    } else {
        decoded_data.resize(result.unwrap(), 0);
        decoded_data
    }
}

pub fn compress(data: &[u8], size: i32) -> Vec<u8> {
    let mut compressor = Compressor::new(CompressionLvl::default());
    let compressed_size = compressor.deflate_compress_bound(size as usize);

    let mut compressed_data = Vec::new();
    compressed_data.resize(compressed_size, 0);

    let actual_sz = compressor.deflate_compress(data, &mut compressed_data).unwrap();
    compressed_data.resize(actual_sz, 0);
    compressed_data
}