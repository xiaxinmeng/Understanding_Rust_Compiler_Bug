plain
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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 632 error codes
* highest error code: E0790
tidy error: /checkout/src/test/incremental/hashes/inherent_impls.rs:144: line longer than 100 chars
tidy error: /checkout/src/test/incremental/hashes/inherent_impls.rs:149: line longer than 100 chars
tidy error: /checkout/src/test/incremental/hashes/inherent_impls.rs:163: line longer than 100 chars
tidy error: /checkout/src/test/incremental/hashes/inherent_impls.rs:168: line longer than 100 chars
tidy error: /checkout/src/test/incremental/hashes/trait_impls.rs:153: line longer than 100 chars
tidy error: /checkout/src/test/incremental/hashes/trait_impls.rs:158: line longer than 100 chars
tidy error: /checkout/src/test/incremental/hashes/trait_impls.rs:191: line longer than 100 chars
tidy error: /checkout/src/test/incremental/hashes/trait_impls.rs:196: line longer than 100 chars
Found 0 error(s) in error codes
Done!
Done!
tidy error: /checkout/compiler/rustc_ast_lowering/src/lib.rs:1467: unexplained "