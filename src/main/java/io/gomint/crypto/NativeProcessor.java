package io.gomint.crypto;

public class NativeProcessor {

    // Constructor and destructor for native structs

    /**
     * Construct new context in native
     *
     * @param side true for encryption and compression, false for decryption and decompression
     * @return reference to the native struct
     */
    static native long createNewContext(boolean side);

    /**
     * Destroy the context given
     *
     * @param ctx which should be destroyed
     */
    static native void destroyContext(long ctx);

}
