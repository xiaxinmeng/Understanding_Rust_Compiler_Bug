text
stappers@trancilo:~/src/rust/RustAVR
$ git clone git@github.com:jsen-/rust-build-core-issue-repro.git
Cloning into 'rust-build-core-issue-repro'...
remote: Enumerating objects: 10, done.
remote: Counting objects: 100% (10/10), done.
remote: Compressing objects: 100% (7/7), done.
remote: Total 10 (delta 0), reused 10 (delta 0), pack-reused 0
Receiving objects: 100% (10/10), done.
stappers@trancilo:~/src/rust/RustAVR
$ cd rust-build-core-issue-repro/
stappers@trancilo:~/src/rust/RustAVR/rust-build-core-issue-repro
$ cargo build --release
   Compiling core v0.0.0 (/home/stappers/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling compiler_builtins v0.1.71
   Compiling rustc-std-workspace-core v1.99.0 (/home/stappers/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling test v0.1.0 (/home/stappers/src/rust/RustAVR/rust-build-core-issue-repro)
WARN rustc_codegen_ssa::back::link Linker does not support -no-pie command line option. Retrying without.
    Finished release [optimized] target(s) in 6.41s
stappers@trancilo:~/src/rust/RustAVR/rust-build-core-issue-repro
$ git grep -i avr
.cargo/config.toml:target = "avr-unknown-gnu-atmega328"
stappers@trancilo:~/src/rust/RustAVR/rust-build-core-issue-repro
$ file target/avr-unknown-gnu-atmega328/release/test.elf
target/avr-unknown-gnu-atmega328/release/test.elf: ELF 32-bit LSB executable,  \
   Atmel AVR 8-bit, version 1 (SYSV), statically linked, with debug_info, not stripped
stappers@trancilo:~/src/rust/RustAVR/rust-build-core-issue-repro
$ 
