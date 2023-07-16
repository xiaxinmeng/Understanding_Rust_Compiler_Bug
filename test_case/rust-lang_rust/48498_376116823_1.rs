
 Documenting openssl v0.9.4 (file:///Users/corey/dev/rust-openssl/openssl)
thread 'rustc' panicked at 'Unexpected type DefId(12/0:31 ~ libc[93ed]::c_int[0])', librustdoc/clean/auto_trait.rs:29:18
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (063deba92 2018-02-23) running on x86_64-apple-darwin

error: Could not document `openssl`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name openssl openssl/src/lib.rs -o /Users/corey/dev/rust-openssl/target/doc -L dependency=/Users/corey/dev/rust-openssl/target/debug/deps --extern openssl_sys=/Users/corey/dev/rust-openssl/target/debug/deps/libopenssl_sys-fc712bbb2e091aa2.rlib --extern bitflags=/Users/corey/dev/rust-openssl/target/debug/deps/libbitflags-c3841cf086926ed6.rlib --extern lazy_static=/Users/corey/dev/rust-openssl/target/debug/deps/liblazy_static-27d6f4600a73ac55.rlib --extern libc=/Users/corey/dev/rust-openssl/target/debug/deps/liblibc-c4a029400d8b931b.rlib --cfg ossl110 --cfg osslconf="OPENSSL_NO_MD2" --cfg osslconf="OPENSSL_NO_RC5" --cfg osslconf="OPENSSL_THREADS" --cfg osslconf="OPENSSL_NO_ASAN" --cfg osslconf="OPENSSL_NO_CRYPTO_MDEBUG" --cfg osslconf="OPENSSL_NO_CRYPTO_MDEBUG_BACKTRACE" --cfg osslconf="OPENSSL_NO_EGD" --cfg osslconf="OPENSSL_NO_FUZZ_AFL" --cfg osslconf="OPENSSL_NO_FUZZ_LIBFUZZER" --cfg osslconf="OPENSSL_NO_HEARTBEATS" --cfg osslconf="OPENSSL_NO_MSAN" --cfg osslconf="OPENSSL_NO_SCTP" --cfg osslconf="OPENSSL_NO_SSL_TRACE" --cfg osslconf="OPENSSL_NO_SSL3" --cfg osslconf="OPENSSL_NO_SSL3_METHOD" --cfg osslconf="OPENSSL_NO_UBSAN" --cfg osslconf="OPENSSL_NO_UNIT_TEST" --cfg osslconf="OPENSSL_NO_WEAK_SSL_CIPHERS" --cfg osslconf="OPENSSL_NO_AFALGENG" --cfg osslconf="OPENSSL_CPUID_OBJ"` (exit code: 101)
