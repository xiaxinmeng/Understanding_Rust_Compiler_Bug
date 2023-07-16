plain
  Downloaded cranelift-frontend v0.85.3
  Downloaded cranelift-bforest v0.85.3
    Checking cfg-if v1.0.0
   Compiling autocfg v1.1.0
   Compiling cranelift-isle v0.85.3
   Compiling log v0.4.14
    Checking hashbrown v0.12.3
   Compiling libc v0.2.126
   Compiling target-lexicon v0.12.3
   Compiling target-lexicon v0.12.3
    Checking byteorder v1.4.3
    Checking slice-group-by v0.3.0
    Checking smallvec v1.8.1
    Checking cranelift-entity v0.85.3
   Compiling crc32fast v1.3.2
   Compiling memchr v2.4.1
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Finished release [optimized] target(s) in 20.67s
Checking stage0 cranelift artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
    Checking cfg-if v1.0.0
   Compiling autocfg v1.1.0
   Compiling cranelift-isle v0.85.3
   Compiling cranelift-codegen-shared v0.85.3
    Checking hashbrown v0.12.3
    Checking byteorder v1.4.3
   Compiling target-lexicon v0.12.3
   Compiling target-lexicon v0.12.3
    Checking smallvec v1.8.1
    Checking slice-group-by v0.3.0
    Checking cranelift-entity v0.85.3
   Compiling memchr v2.4.1
   Compiling crc32fast v1.3.2
    Checking once_cell v1.10.0
---
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
Done!
tidy error: could not find exception package `regalloc`
Remove from EXCEPTIONS list if it is no longer used.
* ahash 0.7.6 (registry+https://github.com/rust-lang/crates.io-index)
* byteorder 1.4.3 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: invalid license `Apache-2.0 WITH LLVM-exception` in `cranelift-isle 0.85.3 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `Apache-2.0 WITH LLVM-exception` in `regalloc2 0.2.3 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: could not find allowed package `regalloc`
* cranelift-isle 0.85.3 (registry+https://github.com/rust-lang/crates.io-index)
Remove from PERMITTED_DEPENDENCIES list if it is no longer used.
tidy error: could not find allowed package `rustc-hash`
Remove from PERMITTED_DEPENDENCIES list if it is no longer used.
tidy error: Dependencies for cranelift not explicitly permitted:
* fxhash 0.2.1 (registry+https://github.com/rust-lang/crates.io-index)
* getrandom 0.2.6 (registry+https://github.com/rust-lang/crates.io-index)
* regalloc2 0.2.3 (registry+https://github.com/rust-lang/crates.io-index)
* slice-group-by 0.3.0 (registry+https://github.com/rust-lang/crates.io-index)
* version_check 0.9.4 (registry+https://github.com/rust-lang/crates.io-index)
* wasi 0.10.2+wasi-snapshot-preview1 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed
Build completed unsuccessfully in 0:00:11
