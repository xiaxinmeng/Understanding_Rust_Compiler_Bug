
     30686:	opening file=/usr/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so [0]; direct_opencount=1
     30686:	
     30686:	symbol=__rustc_codegen_backend;  lookup in file=/usr/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so [0]
     30686:	binding file /usr/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so [0] to /usr/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so [0]: normal symbol `__rustc_codegen_backend'
error: incremental compilation: could not create session directory lock file: Operation not permitted (os error 1)

thread 'main' panicked at 'src/librustc/session/mod.rs:802: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:47:26
note: Run with `RUST_BACKTRACE=1` for a backtrace.
