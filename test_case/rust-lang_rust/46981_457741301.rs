
% entr -r cargo test <<</tmp/file
   Compiling proc-macro2 v0.4.26
   Compiling cc v1.0.28
   Compiling libc v0.2.48
   Compiling unicode-xid v0.1.0
   Compiling autocfg v0.1.2
   Compiling serde v1.0.85
   Compiling semver-parser v0.7.0
   Compiling ryu v0.2.7
thread '<unnamed>' panicked at 'failed to acquire jobserver token: early EOF on jobserver pipe', src/librustc_codegen_ssa/back/write.rs:1348:29
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: failed to acquire jobserver token: early EOF on jobserver pipe

error: aborting due to previous error

thread '<unnamed>' panicked at 'failed to acquire jobserver token: early EOF on jobserver pipe', src/librustc_codegen_ssa/back/write.rs:1348:29
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: failed to acquire jobserver token: early EOF on jobserver pipe

error: Could not compile `unicode-xid`.
warning: build failed, waiting for other jobs to finish...
error: aborting due to previous error

thread '<unnamed>' panicked at 'failed to acquire jobserver token: early EOF on jobserver pipe', src/librustc_codegen_ssa/back/write.rs:1348:29
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: Could not compile `libc`.
warning: build failed, waiting for other jobs to finish...
...
