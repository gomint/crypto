package io.gomint.crypto;

import io.netty.buffer.ByteBuf;
import io.netty.buffer.Unpooled;

public class Processor {

  static {
    System.loadLibrary("crypto");
  }

  private final long ctx;

  public Processor(boolean encryptionModeToggle, byte[] key, byte[] iv) {
    this.ctx = NativeProcessor.createNewContext(encryptionModeToggle, key, iv);
  }

  public ByteBuf process(ByteBuf data) {
    // Ensure that we hold the input buffer until we are done
    data.retain();

    try {
      long pointerAddress = data.memoryAddress();
      int size = data.readableBytes();

      SizedMemoryPointer dataPointer = new SizedMemoryPointer(pointerAddress, size);
      SizedMemoryPointer processedDataPointer = NativeProcessor.process(this.ctx, dataPointer);

      return Unpooled.wrappedBuffer(processedDataPointer.getAddress(), processedDataPointer.getSize(), true);
    } finally {
      data.release(); // Release the recycle safeguard
    }
  }

}
