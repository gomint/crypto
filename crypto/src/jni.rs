// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jlong, jboolean};

use crate::context::{create_context, destroy_context};

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_createNewContext(env: JNIEnv, class: JClass, side: jboolean) -> jlong {
    create_context(side != 0)
}

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_destroyContext(env: JNIEnv, class: JClass, ctx: jlong) {
    destroy_context(ctx)
}