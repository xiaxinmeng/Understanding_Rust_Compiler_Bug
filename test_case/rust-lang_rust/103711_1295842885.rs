
$ rm -rf target/ && RUSTFLAGS="-C force-frame-pointers=yes" cargo +nightly-2022-08-12-x86_64-unknown-linux-gnu build -Z build-std --release --target x86_64-unknown-linux-gnu && objdump -C -d $(find . -name clobbered-frame-pointer -type f) | grep -E '>:$|\$0x1,%bpl' | grep -B1 bpl | head -n 50
   Compiling compiler_builtins v0.1.78
   Compiling core v0.0.0 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling libc v0.2.126
   Compiling cc v1.0.73
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std)
   Compiling unwind v0.0.0 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
   Compiling rustc-std-workspace-alloc v1.99.0 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-alloc)
   Compiling panic_unwind v0.0.0 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/panic_unwind)
   Compiling panic_abort v0.0.0 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/panic_abort)
   Compiling gimli v0.25.0
   Compiling std_detect v0.1.5 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect)
   Compiling miniz_oxide v0.4.0
   Compiling object v0.26.2
   Compiling hashbrown v0.12.3
   Compiling addr2line v0.16.0
   Compiling proc_macro v0.0.0 (/home/sol/.rustup/toolchains/nightly-2022-08-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro)
   Compiling clobbered-frame-pointer v0.1.0 (/home/sol/clobbered-frame-pointer)
    Finished release [optimized] target(s) in 18.20s
$ 
