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
* highest error code: E0788
Found 505 error codes
Found 0 error(s) in error codes
Done!
tidy error: Dependencies not explicitly permitted:
* autocfg 1.1.0 (registry+https://github.com/rust-lang/crates.io-index)
* core-foundation-sys 0.8.3 (registry+https://github.com/rust-lang/crates.io-index)
* crossbeam-channel 0.5.4 (registry+https://github.com/rust-lang/crates.io-index)
* crossbeam-deque 0.8.1 (registry+https://github.com/rust-lang/crates.io-index)
* crossbeam-epoch 0.9.8 (registry+https://github.com/rust-lang/crates.io-index)
* either 1.6.1 (registry+https://github.com/rust-lang/crates.io-index)
* hermit-abi 0.1.19 (registry+https://github.com/rust-lang/crates.io-index)
* memoffset 0.6.5 (registry+https://github.com/rust-lang/crates.io-index)
* ntapi 0.3.7 (registry+https://github.com/rust-lang/crates.io-index)
* num_cpus 1.13.1 (registry+https://github.com/rust-lang/crates.io-index)
* rayon 1.5.3 (registry+https://github.com/rust-lang/crates.io-index)
* rayon-core 1.9.3 (registry+https://github.com/rust-lang/crates.io-index)
* scopeguard 1.1.0 (registry+https://github.com/rust-lang/crates.io-index)
* sysinfo 0.24.2 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed
Build completed unsuccessfully in 0:00:13
