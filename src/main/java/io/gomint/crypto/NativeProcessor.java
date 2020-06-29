package io.gomint.crypto;

public class NativeProcessor {

    // Constructor and destructor for native structs

    /**
     * Construct new context in native
     *
     * @param encryptionModeToggle true for encryption and compression, false for decryption and decompression
     * @param key to use for encrypt / decrypt
     * @param iv to use for init of random data
     * @return reference to the native struct
     */
    static native long createNewContext(boolean encryptionModeToggle, byte[] key, byte[] iv);

    /**
     * Destroy the context given
     *
     * @param ctx which should be destroyed
     */
    static native void destroyContext(long ctx);

    /**
     * Process given data
     *
     * @param ctx with which we want to process given data
     * @param pointer which holds the data
     * @return pointer which holds the processed data
     */
    static native SizedMemoryPointer process(long ctx, SizedMemoryPointer pointer);

}
