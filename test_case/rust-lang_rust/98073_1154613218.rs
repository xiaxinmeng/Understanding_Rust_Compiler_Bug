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
tidy error: /checkout/compiler/rustc_const_eval/src/interpret/intrinsics/validity_invariants_of.rs:18: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_const_eval/src/interpret/intrinsics/validity_invariants_of.rs:36: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_const_eval/src/interpret/intrinsics/validity_invariants_of.rs:108: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_const_eval/src/interpret/intrinsics/validity_invariants_of.rs:117: TODO is deprecated; use FIXME
tidy error: /checkout/library/core/src/intrinsics.rs:2437: TODO is deprecated; use FIXME
tidy error: /checkout/library/core/src/intrinsics.rs:2445: TODO is deprecated; use FIXME
some tidy checks failed
Build completed unsuccessfully in 0:00:10
