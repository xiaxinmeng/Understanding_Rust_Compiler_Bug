
cargo build --release
   Compiling abc v0.1.0 (/tmp/tmp.dgm1dFPHR4)
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-523b587517d366cb.so(+0x5083e3)[0x7f0b5e7123e3]
/lib/x86_64-linux-gnu/libc.so.6(+0x46510)[0x7f0b5dc95510]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(_ZNK4llvm8Constant21containsPoisonElementEv+0x6b)[0x7f0b5bdacd9b]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(+0x2345774)[0x7f0b5aec4774]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(+0x23413d9)[0x7f0b5aec03d9]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(_ZN4llvm16ConstantFoldCallEPKNS_8CallBaseEPNS_8FunctionENS_8ArrayRefIPNS_8ConstantEEEPKNS_17TargetLibraryInfoE+0x101)[0x7f0b5aebf121]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(_ZN4llvm12SimplifyCallEPNS_8CallBaseERKNS_13SimplifyQueryE+0x6e0)[0x7f0b5aeb7990]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(+0x2332181)[0x7f0b5aeb1181]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(_ZN4llvm19SimplifyInstructionEPNS_11InstructionERKNS_13SimplifyQueryEPNS_25OptimizationRemarkEmitterE+0xa5)[0x7f0b5aeb0f35]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(+0x232d61c)[0x7f0b5aeac61c]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(+0x38688ac)[0x7f0b5c3e78ac]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(_ZN4llvm13FPPassManager13runOnFunctionERNS_8FunctionE+0x307)[0x7f0b5ac88e07]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(+0x272559c)[0x7f0b5b2a459c]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(_ZN4llvm6legacy15PassManagerImpl3runERNS_6ModuleE+0x36c)[0x7f0b5bacce6c]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.57.0-nightly.so(LLVMRunPassManager+0xa)[0x7f0b5bdc200a]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-523b587517d366cb.so(+0x20766f1)[0x7f0b602806f1]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-523b587517d366cb.so(+0x207a3ad)[0x7f0b602843ad]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-523b587517d366cb.so(+0x20b9be8)[0x7f0b602c3be8]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-523b587517d366cb.so(+0x20ecf0c)[0x7f0b602f6f0c]
/home/marcel/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/libstd-d31eae41cd792517.so(rust_metadata_std_f70e4442901790c2+0xaa8a3)[0x7f0b5df458a3]
/lib/x86_64-linux-gnu/libc.so.6(+0x988d7)[0x7f0b5dce78d7]
/lib/x86_64-linux-gnu/libc.so.6(+0x129510)[0x7f0b5dd78510]
error: could not compile `abc`

Caused by:
  process didn't exit successfully: `rustc --crate-name abc --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C embed-bitcode=no -C metadata=7e9cc9164c2a8ca5 -C extra-filename=-7e9cc9164c2a8ca5 --out-dir /tmp/tmp.dgm1dFPHR4/target/release/deps -L dependency=/tmp/tmp.dgm1dFPHR4/target/release/deps` (signal: 11, SIGSEGV: invalid memory reference)
