package io.gomint.crypto;

public class NativeProcessor {

    // Constructor and destructor for native structs

    /**
     * Construct new context in native
     *
     * @param encryptionModeToggle true for encryption and compression, false for decryption and decompression
     * @return reference to the native struct
     */
    static native long createNewContext(boolean encryptionModeToggle);

    /**
     * Enable the cryptographic part of the extension
     *
     * @param ctx which should be enabled for crypto
     * @param key to use for encrypt / decrypt
     * @param iv to use for init of random data
     */
    static native void enableCrypto(long ctx, byte[] key, byte[] iv);

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

    /**
     * Enable or disable debug (this also gives performance metrics)
     *
     * @param ctx for which we want to change the debug flag
     * @param debug true when enabled, false otherwise
     */
    static native void debug(long ctx, boolean debug);

}
