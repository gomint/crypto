package io.gomint.crypto;

import io.netty.buffer.ByteBuf;
import io.netty.buffer.Unpooled;

public class Processor {

  private static final NativeCode LOADER = new NativeCode("crypto");

  static {
    if ( !LOADER.load() ) {
      throw new RuntimeException("Could not load crypto native extension");
    }
  }

  private final long ctx;

  public Processor(boolean encryptionModeToggle) {
    this.ctx = NativeProcessor.createNewContext(encryptionModeToggle);
  }

  public void enableCrypto(byte[] key, byte[] iv) {
    NativeProcessor.enableCrypto(this.ctx, key, iv);
  }

  public void debug(boolean debug) {
    NativeProcessor.debug(this.ctx, debug);
  }

  public ByteBuf process(ByteBuf data) {
    try {
      long pointerAddress = data.memoryAddress() + data.readerIndex();
      int size = data.readableBytes();

      SizedMemoryPointer dataPointer = new SizedMemoryPointer(pointerAddress, size);
      SizedMemoryPointer processedDataPointer = NativeProcessor.process(this.ctx, dataPointer);

      return Unpooled.wrappedBuffer(processedDataPointer.getAddress(), processedDataPointer.getSize(), true);
    } finally {
      data.release(); // Release the input since we are done with it and don't need it anymore
    }
  }

}
