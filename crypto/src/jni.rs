use jni::JNIEnv;

use jni::objects::{JClass, JValue};
use jni::sys::{jlong, jboolean, jobject, jbyteArray, jint};

use crate::compression::{compress, decompress};
use std::{mem, slice};
use crate::encryption::{CryptoT, Crypto};

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_createNewContext(_env: JNIEnv, _class: JClass, encryption_mode_toggle: jboolean) -> jlong {
    let ctx = Box::new(Crypto {
        encryption_mode_toggle: encryption_mode_toggle != 0,
        counter: 0,
        key: None,
        aes: None,
        debug: false,
        prealloc_size: 2*1024*1024,
    });

    let a = ctx.as_ref() as *const Crypto;

    mem::forget(ctx);
    a as i64
}

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_enableCrypto(env: JNIEnv, _class: JClass, ctx: jlong, key: jbyteArray, iv: jbyteArray) {
    let key_vec = env.convert_byte_array(key).unwrap();
    let iv_vec = env.convert_byte_array(iv).unwrap();

    let raw_ptr =  ctx as *mut Crypto;
    let context: &mut Crypto = unsafe{ raw_ptr.as_mut().unwrap() };

    context.init_state(key_vec.as_slice(), iv_vec.as_slice());
}

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_destroyContext(_env: JNIEnv, _class: JClass, ctx: jlong) {
    let raw_ptr =  ctx as *mut Crypto;
    mem::drop(raw_ptr)
}

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_debug(_env: JNIEnv, _class: JClass, ctx: jlong, debug_mode: jboolean) {
    let raw_ptr =  ctx as *mut Crypto;
    let context: &mut Crypto = unsafe{ raw_ptr.as_mut().unwrap() };
    context.debug = debug_mode != 0;
}

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_preallocSize(_env: JNIEnv, _class: JClass, ctx: jlong, prealloc_size: jint) {
    let raw_ptr =  ctx as *mut Crypto;
    let context: &mut Crypto = unsafe{ raw_ptr.as_mut().unwrap() };
    context.prealloc_size = prealloc_size as usize;
}

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_process(env: JNIEnv, _class: JClass, ctx: jlong, memory_pointer: jobject) -> jobject {
    // Get the input address and size
    let res_mem_address = env.call_method(memory_pointer, "getAddress", "()J", &[]);
    let mem_address: i64 = res_mem_address.unwrap().j().unwrap();

    let res_size = env.call_method(memory_pointer, "getSize", "()I", &[]);
    let size: i32 = res_size.unwrap().i().unwrap();

    // Build &[u8] from the given memory pointer and size
    let data: &mut [u8] = unsafe { slice::from_raw_parts_mut(mem_address as *mut u8, size as usize) };

    let result_ptr: *const u8;
    let result_size: usize;

    // Get the context which called
    let raw_ptr =  ctx as *mut Crypto;
    let context: &mut Crypto = unsafe{ raw_ptr.as_mut().unwrap() };
    if context.encryption_mode_toggle {
        // Compress first then encrypt
        if context.debug {
            let mut start = std::time::Instant::now();
            let mut compressed = compress(data, size);
            println!("compression of {:?} bytes took {:?}", size, start.elapsed());
            let compressed_size = compressed.len();
            start = std::time::Instant::now();
            let processed = context.process(compressed.as_mut_slice());
            println!("encryption of {:?} bytes took {:?}", compressed_size, start.elapsed());
            result_ptr = processed.as_ptr();
            result_size = processed.len();
            mem::forget(processed);
        } else {
            let mut compressed = compress(data, size);
            let processed = context.process(compressed.as_mut_slice());

            result_ptr = processed.as_ptr();
            result_size = processed.len();
            mem::forget(processed);
        }
    } else {
        // Decrypt first then decompress
        if context.debug {
            let mut start = std::time::Instant::now();
            let decrypted = context.process(data);
            println!("decryption of {:?} bytes took {:?}", size, start.elapsed());
            let compressed_size = decrypted.len();
            start = std::time::Instant::now();
            let decompressed = decompress(decrypted.as_slice(), context.prealloc_size);
            println!("decompression of {:?} bytes took {:?}", compressed_size, start.elapsed());
            result_ptr = decompressed.as_ptr();
            result_size = decompressed.len();
            mem::forget(decompressed);
        } else {
            let decrypted = context.process(data);
            let decompressed = decompress(decrypted.as_slice(), context.prealloc_size);

            result_ptr = decompressed.as_ptr();
            result_size = decompressed.len();
            mem::forget(decompressed);
        }
    }

    // Create response object
    let res_memory_pointer_class = env.get_object_class(memory_pointer);
    if res_memory_pointer_class.is_err() {}

    let arguments: &[JValue] = &[JValue::from(result_ptr as i64), JValue::from(result_size as i32)];
    env.new_object(res_memory_pointer_class.unwrap(), "(JI)V", arguments).unwrap().into_inner()
}