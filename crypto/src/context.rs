
use sha2::Sha256;
use libdeflater::{Compressor, Decompressor};

#[repr(C)]
pub(crate) struct Context {
    // Generics
    pub(crate) encryption_mode_toggle: bool,
    pub(crate) debug: bool,

    // Encryption
    pub(crate) counter: i64,
    pub(crate) aes: Option<ctr::Ctr128BE<aes::Aes256>>,
    pub(crate) key: Option<Vec<u8>>,
    pub(crate) digest: Sha256,

    // Compression
    pub(crate) prealloc_size: usize,
    pub(crate) compressor: Option<Compressor>,
    pub(crate) decompressor: Option<Decompressor>,
}

impl AsMut<Context> for Context {
    fn as_mut(&mut self) -> &mut Context {
        self
    }
}