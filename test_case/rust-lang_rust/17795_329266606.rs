sh
#!/usr/bin/env sh

set -eu

LLVM_CONFIG_PATH=$(find build -name llvm-config -type f | head -n1)
CXX_FLAGS=$($LLVM_CONFIG_PATH --cxxflags)

echo CXX_FLAGS: $CXX_FLAGS

RUST_BACKTRACE=1 bindgen $@ src/rustllvm/rustllvm.h -o src/librustc_llvm/ffi.rs -- $CXX_FLAGS
