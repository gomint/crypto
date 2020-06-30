package io.gomint.crypto;

import io.netty.buffer.ByteBuf;
import io.netty.buffer.PooledByteBufAllocator;
import java.util.concurrent.ThreadLocalRandom;

public class TestRunner {

  public static void main(String[] args) {
    System.setProperty( "io.netty.tryReflectionSetAccessible","true");

    byte[] key = new byte[32];
    ThreadLocalRandom.current().nextBytes(key);

    byte[] iv = new byte[16];
    ThreadLocalRandom.current().nextBytes(iv);

    Processor processor = new Processor(true);
    Processor processor_decrypt = new Processor(false);

    ByteBuf buf = PooledByteBufAllocator.DEFAULT.directBuffer();
    buf.writeInt(187);
    buf.writeBytes("TEST".getBytes());
    ByteBuf out = processor.process(buf);
    ByteBuf check = processor_decrypt.process(out);

    System.out.println(check.readInt());
    check.release();

    processor.enableCrypto(key, iv);
    processor_decrypt.enableCrypto(key, iv);

    buf = PooledByteBufAllocator.DEFAULT.directBuffer();
    buf.writeInt(187);
    buf.writeBytes("TEST".getBytes());
    out = processor.process(buf);
    processor_decrypt.process(out);
  }

}
