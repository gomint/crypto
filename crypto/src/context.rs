use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::borrow::Borrow;
use std::sync::Mutex;

lazy_static! {
    static ref CONTEXTS: Mutex<HashMap<i64, Context>> = Mutex::new(HashMap::new());
    static ref COUNTER: AtomicI64 = AtomicI64::new(0);
}

struct Context {
    side: bool,
    counter: i64,
}

pub fn create_context(side: bool) -> i64 {
    let ctx = Context {
        side,
        counter: 0,
    };

    let index = COUNTER.fetch_add(1, Ordering::SeqCst);
    CONTEXTS.lock().unwrap().insert(index, ctx);
    index
}

pub fn destroy_context(ctx: i64) {
    CONTEXTS.lock().unwrap().remove(&ctx);
}