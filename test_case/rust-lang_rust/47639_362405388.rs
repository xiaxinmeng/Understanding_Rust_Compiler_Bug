
$ cargo +21882aad7299e8e859785845ac12374990f24dae test --frozen --doc
   Compiling constant_time_eq v0.1.3
   Compiling etcommon-hexutil v0.2.3
   Compiling rustc-serialize v0.3.24
   Compiling libc v0.2.36
   Compiling gcc v0.3.54
   Compiling serde v1.0.27
   Compiling dtoa v0.4.2
   Compiling typenum v1.9.0
   Compiling byte-tools v0.2.0
   Compiling odds v0.2.26
   Compiling num-traits v0.1.42
   Compiling nodrop v0.1.12
   Compiling itoa v0.3.4
   Compiling fake-simd v0.1.2
   Compiling rand v0.4.2
   Compiling secp256k1-test v0.7.2
   Compiling arrayvec v0.3.25
   Compiling generic-array v0.8.3
   Compiling block-buffer v0.2.0
   Compiling digest v0.6.2
   Compiling crypto-mac v0.4.0
   Compiling hmac v0.4.2
   Compiling sha2 v0.6.0
   Compiling hmac-drbg v0.1.2
   Compiling libsecp256k1 v0.1.13 (file:///.../libsecp256k1)
   Compiling serde_json v1.0.9
    Finished dev [unoptimized + debuginfo] target(s) in 31.2 secs
   Doc-tests secp256k1
thread '<unnamed>' panicked at 'librustc/lint/context.rs:1049: failed to process buffered lint here', librustc/session/mod.rs:1170:26
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: test failed, to rerun pass '--doc'
