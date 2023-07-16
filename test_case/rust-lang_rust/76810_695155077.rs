
  --- stderr
  llvm-config: error: libLLVM-11-rust-1.48.0-nightly.so is missing
  thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-musl/llvm/build/bin/llvm-config" "--link-shared" "--libs" "--system-libs" "aarch64" "arm" "asmparser" "avr" "bitreader" "bitwriter" "coverage" "hexagon" "instrumentation" "ipo" "linker" "lto" "mips" "msp430" "nvptx" "powerpc" "riscv" "sparc" "systemz" "webassembly" "x86"
  expected success, got: exit code: 1', src/build_helper/lib.rs:139:9
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
