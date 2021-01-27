package io.gomint.crypto;

import io.gomint.nativeloader.NativeLoader;
import io.netty.buffer.ByteBuf;
import io.netty.buffer.PooledByteBufAllocator;
import io.netty.buffer.Unpooled;
import oshi.PlatformEnum;

public class Processor {

  static {
    if ( !NativeLoader.create()
            .supports(PlatformEnum.WINDOWS, "amd64")
            .supports(PlatformEnum.LINUX, "amd64")
            .supports(PlatformEnum.LINUX, "arm")
            .supports(PlatformEnum.MACOS, "aarch64")
            .load("crypto", Processor.class.getClassLoader()) ) {
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

  public void preallocSize(int size) {
    NativeProcessor.preallocSize(this.ctx, size);
  }

  public ByteBuf process(ByteBuf data) {
    try {
      long pointerAddress = data.memoryAddress() + data.readerIndex();
      int size = data.readableBytes();

      SizedMemoryPointer dataPointer = new SizedMemoryPointer(pointerAddress, size);
      SizedMemoryPointer processedDataPointer = NativeProcessor.process(this.ctx, dataPointer);

      if (processedDataPointer.getAddress() == 0 || processedDataPointer.getSize() == 0) {
        return PooledByteBufAllocator.DEFAULT.directBuffer();
      }

      return Unpooled.wrappedBuffer(processedDataPointer.getAddress(), processedDataPointer.getSize(), true);
    } finally {
      data.release(); // Release the input since we are done with it and don't need it anymore
    }
  }

}
