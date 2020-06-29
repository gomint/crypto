use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Mutex;
use crate::encryption::{Crypto, CryptoT};

lazy_static! {
    static ref CONTEXTS: Mutex<HashMap<i64, Crypto>> = Mutex::new(HashMap::new());
    static ref COUNTER: AtomicI64 = AtomicI64::new(0);
}

pub fn create_context(encryption_mode_toggle: bool, key: &[u8], iv: &[u8]) -> i64 {
    let mut ctx = Crypto {
        encryption_mode_toggle,
        counter: 0,
        aes: None,
        key: None,
    };

    ctx.init_state(key, iv);

    let index = COUNTER.fetch_add(1, Ordering::SeqCst);
    CONTEXTS.lock().unwrap().insert(index, ctx);
    index
}

pub fn destroy_context(ctx: i64) {
    CONTEXTS.lock().unwrap().remove(&ctx);
}

pub fn get_context(ctx: i64) -> &'static Crypto {
    CONTEXTS.lock().unwrap().get(&ctx).unwrap()
}