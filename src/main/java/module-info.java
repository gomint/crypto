module gomint.crypto {
  // Java modules
  requires jdk.unsupported;

  requires io.netty.buffer;
  requires oshi.core;
  requires com.google.common;
  requires io.netty.common;

  // Logging modules
  requires slf4j.api;

  exports io.gomint.crypto;
}