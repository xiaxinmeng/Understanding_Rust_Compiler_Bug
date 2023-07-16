
[+] Building for riscv32 SiFive-E series
       Fresh version_check v0.1.4
       Fresh libc v0.2.43
       Fresh ucd-util v0.1.1
       Fresh utf8-ranges v1.0.1
       Fresh bare-metal v0.2.3
       Fresh memchr v2.0.2
       Fresh regex-syntax v0.6.2
       Fresh riscv v0.3.0
       Fresh aho-corasick v0.6.8
       Fresh lazy_static v1.1.0
       Fresh thread_local v0.3.6
       Fresh regex v1.0.5
   Compiling diosix v2.0.0 (/data/rust/rust-lang-rust/issues/misc/diosix)
     Running `/data/rust/rust-lang-rust/issues/misc/diosix/target/release/build/diosix-631a70b0814a0e1c/build-script-build`
error: failed to run custom build command for `diosix v2.0.0 (/data/rust/rust-lang-rust/issues/misc/diosix)`
process didn't exit successfully: `/data/rust/rust-lang-rust/issues/misc/diosix/target/release/build/diosix-631a70b0814a0e1c/build-script-build` (exit code: 101)
--- stdout
cargo:rerun-if-changed=src/platform/riscv32/sifive_e/link.ld
cargo:rerun-if-changed=src/platform/riscv32/sifive_e/asm/entry.s

--- stderr
thread 'main' panicked at 'Failed to assemble src/platform/riscv32/sifive_e/asm/entry.s: Os { code: 2, kind: NotFound, message: "No such file or directory" }', libcore/result.rs:1009:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
