package io.gomint.crypto;

public class NativeProcessor {

    // Constructor and destructor for native structs
    static native long createNewContext();
    static native void destroyContext(long ctx);

}
