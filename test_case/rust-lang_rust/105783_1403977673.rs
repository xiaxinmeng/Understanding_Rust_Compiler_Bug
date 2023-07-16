
readelf -lW $(find ~/.rustup/ -name libLLVM-*-rust-*-stable.so) | grep GNU_STACK
