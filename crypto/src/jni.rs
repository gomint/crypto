use jni::JNIEnv;

use jni::objects::{JClass};
use jni::sys::{jlong, jboolean, jobject, jbyteArray};

use crate::context::{create_context, destroy_context, get_context};
use std::slice::from_raw_parts;

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_createNewContext(env: JNIEnv, class: JClass, encryption_mode_toggle: jboolean, key: jbyteArray, iv: jbyteArray) -> jlong {
    let key_vec = env.convert_byte_array(key).unwrap();
    let iv_vec = env.convert_byte_array(iv).unwrap();

    create_context(encryption_mode_toggle != 0, key_vec.as_slice(), iv_vec.as_slice())
}

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_destroyContext(env: JNIEnv, class: JClass, ctx: jlong) {
    destroy_context(ctx)
}

#[no_mangle]
pub extern "system" fn Java_io_gomint_crypto_NativeProcessor_process(env: JNIEnv, class: JClass, ctx: jlong, memory_pointer: jobject) -> jobject {
    // Get the input address and size
    let res_mem_address = env.call_method(memory_pointer, "getAddress", "()J", &[]);
    let mem_address: i64 = res_mem_address.unwrap().j().unwrap();

    let res_size = env.call_method(memory_pointer, "getSize", "()I", &[]);
    let size: i32 = res_size.unwrap().i().unwrap();

    // Build &[u8] from the given memory pointer and size
    let data: &[u8];
    unsafe {
        data = from_raw_parts(mem_address as *const u8, size as usize);
    }

    // Get the context which called
    let context = get_context(ctx);
    if context.encryption_mode_toggle {

    } else {

    }

    // Create response object
    let res_memory_pointer_class = env.get_object_class(memory_pointer);
    if res_memory_pointer_class.is_err() {

    }

    env.new_object(res_memory_pointer_class.unwrap(), "(JI)V", &[])
}