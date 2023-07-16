plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0785
Found 501 error codes
Found 0 error codes with no tests
Done!
tidy error: Dependencies not explicitly permitted:
* bumpalo 3.7.1 (registry+https://github.com/rust-lang/crates.io-index)
* js-sys 0.3.49 (registry+https://github.com/rust-lang/crates.io-index)
* wasm-bindgen 0.2.72 (registry+https://github.com/rust-lang/crates.io-index)
* wasm-bindgen-backend 0.2.72 (registry+https://github.com/rust-lang/crates.io-index)
* wasm-bindgen-macro 0.2.72 (registry+https://github.com/rust-lang/crates.io-index)
* wasm-bindgen-macro-support 0.2.72 (registry+https://github.com/rust-lang/crates.io-index)
* wasm-bindgen-shared 0.2.72 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
