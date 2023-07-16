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
Found 435 error codes
Found 0 error codes with no tests
Done!
* 316 features
tidy error: crate `rustc-ap-rustc_ast` is duplicated in `Cargo.lock`, it is too expensive to build multiple times, so make sure only one version appears across all dependencies
  * rustc-ap-rustc_ast 691.0.0 (registry+https://github.com/rust-lang/crates.io-index)
  * rustc-ap-rustc_ast 697.0.0 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

