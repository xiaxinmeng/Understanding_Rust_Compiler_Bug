
$ rustc --target msp430 $(rustc --print sysroot)/lib/rustlib/src/rust/src/libcore/lib.rs --emit=asm
Call result #4 has unhandled type i16
UNREACHABLE executed at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/CodeGen/CallingConvLower.cpp:167!
