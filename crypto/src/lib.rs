#[global_allocator]
static GLOBAL: System = std::alloc::System;

use std::alloc::System;

mod encryption;
mod jni;
mod compression;
mod context;