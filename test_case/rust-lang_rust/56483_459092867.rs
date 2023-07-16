
nikic@helios:~/rust-simd-noise$ rustc +ae1ba150a280ebe428c760fbbbd58ae758ba9ea7-alt -V
rustc 1.34.0-nightly (ae1ba150a 2019-01-29)
nikic@helios:~/rust-simd-noise$ cargo +ae1ba150a280ebe428c760fbbbd58ae758ba9ea7-alt build
   Compiling simdeez v0.4.2
   Compiling simdnoise v2.3.6 (/home/nikic/rust-simd-noise)
rustc: /checkout/src/llvm-project/llvm/lib/Target/X86/MCTargetDesc/X86MCCodeEmitter.cpp:118: static uint8_t (anonymous namespace)::X86MCCodeEmitter::ModRMByte(unsigned int, unsigned int, unsigned int): Assertion `Mod < 4 && RegOpcode < 8 && RM < 8 && "ModRM Fields out of range!"' failed.
error: Could not compile `simdnoise`.
