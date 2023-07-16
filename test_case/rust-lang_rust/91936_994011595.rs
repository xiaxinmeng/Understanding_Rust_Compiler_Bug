plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0786
Found 502 error codes
Found 0 error codes with no tests
Done!
tidy error: Dependencies not explicitly permitted:
* block-buffer 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
* cpufeatures 0.2.1 (registry+https://github.com/rust-lang/crates.io-index)
* digest 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
* generic-array 0.14.4 (registry+https://github.com/rust-lang/crates.io-index)
* opaque-debug 0.3.0 (registry+https://github.com/rust-lang/crates.io-index)
* sha2 0.9.8 (registry+https://github.com/rust-lang/crates.io-index)
* typenum 1.14.0 (registry+https://github.com/rust-lang/crates.io-index)
* version_check 0.9.3 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:11
