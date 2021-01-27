module gomint.crypto {
  // Java modules
  requires jdk.unsupported;

  requires io.netty.buffer;
  requires com.github.oshi;
  requires gomint.nativeloader;
  requires io.netty.common;

  exports io.gomint.crypto;
}