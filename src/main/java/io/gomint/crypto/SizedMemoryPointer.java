package io.gomint.crypto;

public class SizedMemoryPointer {
  private final long address;
  private final int size;

  public SizedMemoryPointer(long address, int size) {
    this.address = address;
    this.size = size;
  }

  public long getAddress() {
    return address;
  }

  public int getSize() {
    return size;
  }
}
